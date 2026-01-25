mod shell;

use anyhow::Result;
use tracing::{Level, info};

  fn main() -> Result<()> {
    // Initialize tracing subscriber for logging
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)  // show ERROR and above
        .pretty()
        .init();

    let parent_name = shell::get_parent_process_name()?;
    info!(parent_name = %parent_name, "Parent process name retrieved");
    Ok(())
  }