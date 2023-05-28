use anyhow::Result;
use jsonl::Connection;
use pros_simulator::*;
use pros_simulator_api::*;
use std::panic;
use std::process;
use std::sync::mpsc;
use std::thread::spawn;

fn main() -> Result<()> {
    let orig_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        // invoke the default handler and exit the process
        orig_hook(panic_info);
        process::exit(1);
    }));

    let mut connection = Connection::new_from_stdio();
    let (outgoing_tx, outgoing_rx) = mpsc::channel::<client::Event>();

    let simulator_thread = {
        let outgoing_tx = outgoing_tx.clone();
        spawn(move || {
            run_simulator(outgoing_tx).unwrap();
        })
    };

    let tx_thread = spawn(move || {
        while let Ok(event) = outgoing_rx.recv() {
            connection.write(&event).unwrap();
        }
    });

    simulator_thread.join().unwrap();
    outgoing_tx.send(client::Event::SimulatorStopped).unwrap();

    tx_thread.join().unwrap();

    Ok(())
}

fn run_simulator(tx: mpsc::Sender<client::Event>) -> Result<()> {
    let wasm_path = std::env::args()
        .nth(1)
        .expect("Usage: pros-simulator <wasm file>");
    let wasm = std::fs::read(wasm_path)?;

    let mut robot = Robot::new(&wasm, tx)?;
    robot.initialize()?;

    Ok(())
}
