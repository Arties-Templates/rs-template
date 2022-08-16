use std::env;
use anyhow::Result;

fn main() -> Result<()> {
    let args = env::args().skip(1);

    for arg in args {
      println!("{}", arg)
    }

    Ok(())
}
