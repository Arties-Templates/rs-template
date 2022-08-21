use color_eyre::{eyre::eyre, Report};

fn main() -> Result<(), Report> {
  color_eyre::install()?;

  println!("Hello, World!");

  Ok(())
}
