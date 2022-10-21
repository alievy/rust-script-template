mod util;
mod program;
use program::run;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    run()?;
    Ok(())
}
