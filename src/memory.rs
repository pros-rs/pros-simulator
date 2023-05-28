use crate::*;
use wasmtime::*;

pub type WasmPtr = u32;

pub struct RobotMemory {
    pub wasm_memory: Memory,
    pub wasm_mem_alloc: TypedFunc<WasmPtr, WasmPtr>,
    pub wasm_mem_dealloc: TypedFunc<WasmPtr, ()>,
}

impl RobotMemory {
    pub fn new(mut store: &mut Store<RobotState>, instance: &Instance) -> Self {
        Self {
            wasm_memory: instance
                .get_memory(&mut store, "memory")
                .expect("WASM code must expose its memory"),
            wasm_mem_alloc: instance
                .get_typed_func(&mut store, "mem_alloc")
                .expect("WASM code must expose a `mem_alloc` function"),
            wasm_mem_dealloc: instance
                .get_typed_func(&mut store, "mem_dealloc")
                .expect("WASM code must expose a `mem_free` function"),
        }
    }

    pub fn alloc<'a, T>(
        &'a self,
        mut store: StoreContextMut<'a, T>,
        length: WasmPtr,
    ) -> MemoryHandle<'a, T> {
        let offset = self
            .wasm_mem_alloc
            .call(&mut store, length)
            .expect("allocation of wasm memory failed");
        MemoryHandle {
            location: MemoryLocation { offset, length },
            memory: self,
            store,
            should_dealloc: true,
        }
    }

    /// Similar to [`Self::get`] but deallocates the memory after the handle is dropped.
    pub fn get_owned<'a, T>(
        &'a self,
        store: StoreContextMut<'a, T>,
        location: MemoryLocation,
    ) -> MemoryHandle<'a, T> {
        MemoryHandle {
            location,
            memory: self,
            store,
            should_dealloc: true,
        }
    }

    pub fn get<'a, T>(
        &'a self,
        store: StoreContextMut<'a, T>,
        location: MemoryLocation,
    ) -> MemoryHandle<'a, T> {
        MemoryHandle {
            location,
            memory: self,
            store,
            should_dealloc: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemoryLocation {
    pub offset: WasmPtr,
    pub length: WasmPtr,
}

impl MemoryLocation {
    pub fn new(offset: WasmPtr, length: WasmPtr) -> Self {
        Self { offset, length }
    }

    pub fn as_tuple(&self) -> (WasmPtr, WasmPtr) {
        (self.offset, self.length)
    }

    pub fn as_wasm_tuple(&self) -> (i32, i32) {
        (self.offset as i32, self.length as i32)
    }
}

pub struct MemoryHandle<'a, T> {
    pub location: MemoryLocation,
    pub memory: &'a RobotMemory,
    pub store: StoreContextMut<'a, T>,
    pub should_dealloc: bool,
}

impl<'a, T> MemoryHandle<'a, T> {
    pub fn write(&mut self, data: &[u8], offset: WasmPtr) {
        let data_len: WasmPtr = data.len().try_into().unwrap();
        if data_len + offset > self.location.length {
            panic!("cannot write past the end of a memory fragment");
        }
        let buf = self.memory.wasm_memory.data_mut(&mut self.store);
        for (i, byte) in data.iter().enumerate() {
            buf[self.location.offset as usize + offset as usize + i] = *byte;
        }
    }

    pub fn into_raw(self) -> MemoryLocation {
        let location = self.location.clone();
        std::mem::forget(self);
        location
    }
}

impl<'a, T> AsRef<[u8]> for MemoryHandle<'a, T> {
    fn as_ref(&self) -> &[u8] {
        let end_ptr = self.location.offset + self.location.length;
        &self.memory.wasm_memory.data(&self.store)[self.location.offset as usize..end_ptr as usize]
    }
}

impl<'a, T> From<&MemoryHandle<'a, T>> for Vec<u8> {
    fn from(slice: &MemoryHandle<'a, T>) -> Self {
        slice.as_ref().to_vec()
    }
}

impl<'a, T> ToString for MemoryHandle<'a, T> {
    fn to_string(&self) -> String {
        // TODO: report warning on invalid utf8
        String::from_utf8_lossy(self.as_ref()).to_string()
    }
}

impl<'a, T> From<&MemoryHandle<'a, T>> for (i32, i32) {
    fn from(slice: &MemoryHandle<'a, T>) -> Self {
        (slice.location.offset as i32, slice.location.length as i32)
    }
}

impl<'a, T> From<&MemoryHandle<'a, T>> for (WasmPtr, WasmPtr) {
    fn from(slice: &MemoryHandle<'a, T>) -> Self {
        (slice.location.offset, slice.location.length)
    }
}

impl<'a, T> Drop for MemoryHandle<'a, T> {
    fn drop(&mut self) {
        if self.should_dealloc {
            self.memory
                .wasm_mem_dealloc
                .call(&mut self.store, self.location.offset)
                .expect("deallocation of wasm memory failed");
        }
    }
}
