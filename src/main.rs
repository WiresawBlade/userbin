use std::error::Error;

use tokio::runtime::Runtime;
use userbin::{tempalte::create_ps1, CLI_ARGS};

fn main() -> Result<(), Box<dyn Error>> {
    let rt = Runtime::new().unwrap();
    rt.block_on(create_ps1(CLI_ARGS.target()))?;
    Ok(())
}
