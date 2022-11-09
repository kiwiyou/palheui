use clap::Parser;

/// Simple Ahui to Readable C transpiler
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// source file, enter "-" to read from stdin
    input: String,
}

fn main() {
    let args = Args::parse();
    let content = if args.input == "-" {
        std::io::read_to_string(std::io::stdin()).expect("Could not read from stdin")
    } else {
        std::fs::read_to_string(args.input).expect("Could not read from input file")
    };
    print!("{}", palheui::transpile(&content));
}
