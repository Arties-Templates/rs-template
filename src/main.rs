use color_eyre::{eyre::{eyre, Context}, Report, Help};

fn main() -> Result<(), Report> {
  color_eyre::install()?;

  println!("Hello, World!");

  Ok(())
}
