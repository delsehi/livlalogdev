use livla_log::get_json_from_query;
use std::{fs::File, io::Read};
use tokio_with_wasm::tokio;
mod bridge;
mod messages;
mod sample_functions;

/// This `hub` crate is the entry point for the Rust logic.
/// Always use non-blocking async functions such as `tokio::fs::File::open`.
async fn main() {
    // Repeat `tokio::spawn` anywhere in your code
    // if more concurrent tasks are needed.
    // tokio::spawn(sample_functions::tell_numbers());
    // tokio::spawn(sample_functions::stream_fractal());
    // tokio::spawn(sample_functions::run_debug_tests());
    tokio::spawn(process_query());
}

async fn process_query() {
    use messages::sql_query::*;
    let mut receiver = LogAndQueryInput::get_dart_signal_receiver();
    while let Some(dart_signal) = receiver.recv().await {
        let log = dart_signal.message.log;
        let mut file = File::open(log).unwrap();
        let mut buf = String::new();
        file.read_to_string(&mut buf).unwrap();
        let query = dart_signal.message.query;
        let (json, schema) = get_json_from_query(&buf, &query).await;
        QueryResultOutput { json, schema }.send_signal_to_dart(None);
    }
}
