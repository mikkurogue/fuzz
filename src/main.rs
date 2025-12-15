use clap::Parser;

mod fuzz;

use fuzz::Fuzz;

#[derive(Parser)]
pub struct Cli {
    #[clap(short, long)]
    pub word: String,
}

fn main() {
    let args = Cli::parse();

    if args.word.is_empty() {
        println!("Please provide a target to find");
        std::process::exit(1);
    }

    let fuzz = Fuzz::new(args.word);

    fuzz.run();
}
