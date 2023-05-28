//! ```rust,no_run
//! # use pros_simulator_macros::*;
//! define_api! {
//!     wasm_import_module = pros;
//!
//!     mod crate::api::lcd {
//!         fn lcd_print(line_num: i32, str_len: i32, str_ptr: i32);
//!     }
//! }
//! ```

use std::collections::HashMap;

use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    spanned::Spanned,
    token::{Brace, Paren},
    FnArg, Ident, Path, Result, ReturnType, Token, Type, TypePath,
};

fn primitive_type_to_wasm(primitive: &TypePath) -> TokenStream {
    let span = primitive.span();
    let last = Ident::new(
        primitive
            .path
            .segments
            .last()
            .unwrap()
            .ident
            .to_string()
            .to_uppercase()
            .as_str(),
        span,
    );
    quote_spanned!(span=> ::wasmtime::ValType::#last)
}

#[allow(dead_code)]
struct ApiFn {
    pub fn_token: Token![fn],
    pub ident: Ident,
    pub paren_token: Paren,
    pub inputs: Punctuated<FnArg, Token![,]>,
    pub output: ReturnType,
    pub semi_token: Token![;],
}

impl Parse for ApiFn {
    fn parse(input: ParseStream) -> Result<Self> {
        let fn_token = input.parse()?;
        let ident = input.parse()?;
        let content;
        let paren_token = parenthesized!(content in input);
        let inputs = content.parse_terminated(FnArg::parse, Token![,])?;
        let output = input.parse()?;
        let semi_token = input.parse()?;
        Ok(Self {
            fn_token,
            ident,
            paren_token,
            inputs,
            output,
            semi_token,
        })
    }
}

impl ApiFn {
    /// Convert primitive types (e.g. i32) to Wasmtime tyoes (e.g. wasmtime::TypeVar::I32)
    pub fn arg_types_as_wasm(&self) -> Vec<TokenStream> {
        self.inputs
            .iter()
            .filter_map(|arg| match arg {
                FnArg::Typed(pat_type) => Some(&pat_type.ty),
                _ => None,
            })
            .map(|ty| match **ty {
                Type::Path(ref type_path) => primitive_type_to_wasm(type_path),
                _ => unimplemented!("unexpected type variant - try a primitive like i64"),
            })
            .map(Into::<TokenStream>::into)
            .collect()
    }
}

#[allow(dead_code)]
struct ApiMod {
    pub mod_token: Token![mod],
    pub path: Path,
    pub brace_token: Brace,
    pub fns: Vec<ApiFn>,
}

impl Parse for ApiMod {
    fn parse(input: ParseStream) -> Result<Self> {
        let mod_token = input.parse()?;
        let path = input.parse()?;
        let content;
        let brace_token = syn::braced!(content in input);

        let mut fns = Vec::new();
        while !content.is_empty() {
            let fn_item = content.parse::<ApiFn>()?;
            fns.push(fn_item);
        }

        Ok(Self {
            mod_token,
            path,
            brace_token,
            fns,
        })
    }
}

#[derive(Clone)]
#[allow(dead_code)]
struct ApiDirective {
    pub name: Ident,
    pub eq_token: Token![=],
    pub path: Ident,
    pub semi_token: Token![;],
}

impl Parse for ApiDirective {
    fn parse(input: ParseStream) -> Result<Self> {
        let name = input.parse()?;
        let eq_token = input.parse()?;
        let path = input.parse()?;
        let semi_token = input.parse()?;
        Ok(Self {
            name,
            eq_token,
            path,
            semi_token,
        })
    }
}

struct Api {
    pub directives: Vec<ApiDirective>,
    pub implementation_modules: Vec<ApiMod>,
}

impl Parse for Api {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut directives = Vec::new();
        while !input.is_empty() {
            let Ok(directive) = input.parse() else {
                break;
            };
            directives.push(directive);
        }
        let mut implementation_modules = Vec::new();
        while !input.is_empty() {
            let Ok(mod_item) = input.parse() else {
                break;
            };
            implementation_modules.push(mod_item);
        }
        Ok(Self {
            directives,
            implementation_modules,
        })
    }
}

impl Api {
    pub fn directives(&self) -> HashMap<String, Ident> {
        self.directives
            .iter()
            .cloned()
            .map(|directive| (directive.name.to_string(), directive.path))
            .collect()
    }
}

fn define_api_impl(input: TokenStream) -> TokenStream {
    let input = match syn::parse2::<Api>(input) {
        Ok(data) => data,
        Err(err) => {
            return err.to_compile_error();
        }
    };

    let mut directives = input.directives();
    let wasm_import_module = directives
        .remove("wasm_import_module")
        .map(|ident| ident.to_string())
        .unwrap_or_else(|| "pros".to_string());

    let mut exports = TokenStream::new();

    for implementation_module in input.implementation_modules {
        for func in implementation_module.fns {
            // wasm "types" are enum variants that can be used with a linker to export a function signature to the simulated code
            let wasm_types = func.arg_types_as_wasm();
            let ApiFn { ident, output, .. } = func;

            // although wasm functions can have multiple return values we'll only be using 1
            let ret_type: proc_macro2::TokenStream = match output {
                ReturnType::Default => TokenStream::new(),
                ReturnType::Type(_, ty) => match *ty {
                    Type::Path(type_path) => primitive_type_to_wasm(&type_path),
                    _ => unimplemented!("unexpected type variant - try a primitive like i64"),
                },
            };
            let impl_path = &implementation_module.path;
            let fn_name = ident.to_string();
            let host_export = quote! {
                (#wasm_import_module, #fn_name) => {
                    let prototype = ::wasmtime::FuncType::new([
                        #(#wasm_types),*
                    ], [
                        #ret_type
                    ]);
                    linker.func_new(
                        #wasm_import_module,
                        #fn_name,
                        prototype,
                        #impl_path::#ident
                    )?;
                }
            };

            exports.extend(host_export);
        }
    }

    quote! {
        pub fn link_api<'a>(linker: &mut wasmtime::Linker<State>, module: &wasmtime::Module) -> Result<(), anyhow::Error> {
            for import in module.imports() {
                match (import.module(), import.name()) {
                    #exports
                    _ => {}
                }
            }

            Ok(())
        }
    }
}

/// Create an interface for the simulator (host) and simulation (client) to communicate.
///
/// On WASM targets, this creates `pub extern "C"` bindings for each function.
/// On non-WASM target, this emits a function `link_api` that uses a Wasmtime linker to
/// make the functions available to the simulation.
///
/// Directives take the form `name = value;` and come at the beginning of the block.
/// Functions are contained in modules like this: `mod crate::api::lcd {...}`. Modules do
/// not serve any purpose except to mark where the function implementations should be searched for.
#[proc_macro]
pub fn define_api(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    define_api_impl(input.into()).into()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse() {
        let input = quote! {
            wasm_import_module = pros;

            mod crate::api::lcd {
                fn print(text_ptr: i32);
            }

            mod crate::api::motor {
                fn get_speed(id: i64) -> f64;
            }
        };

        _ = syn::parse2::<Api>(input).unwrap();
    }

    #[test]
    fn codegen() {
        let input = quote! {
            wasm_import_module = pros;

            mod crate::api::lcd {
                fn print(text_ptr: i32);
            }

            mod crate::api::motor {
                fn get_speed(id: i64) -> f64;
            }
        };

        _ = define_api_impl(input);
    }
}
