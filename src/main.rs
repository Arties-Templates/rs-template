use std::env;

fn main() {
    let args = env::args().skip(1);

    for arg in args {
      println!("{}", arg)
    }
}
