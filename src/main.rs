use clap::Parser;

mod fuzz;

#[derive(Parser)]
pub struct Cli {}

fn main() {
    let fuzz = fuzz::Fuzz::new("sample idea".to_string());

    fuzz.run();
}
