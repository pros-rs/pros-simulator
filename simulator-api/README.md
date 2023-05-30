# pros-simulator-api

> Serde type definitions for controlling the pros-rs simulator

## About

The pros-rs simulator uses the JSONL format over stdin/stdout to communicate with any apps using it. Each line sent through the simulator's stdout can be deserialized to a `pros_simulator_api::client::Event`.
