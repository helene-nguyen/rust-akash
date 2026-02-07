mod shell;
mod store;

// TODO REMOVE: This is just for testing the store output, should be moved to a proper test
#[path = "../test_output/store_output.rs"]
mod store_output;

use anyhow::Result;
use tracing::{Level};
// use store::AliasStore;
use store_output::run_store_output;


fn main() -> Result<()> {
    // Initialize tracing (adjust based on your setup)
    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .init();

    run_store_output()?;
    Ok(())
}
