use clap::Parser;
use clipse::encryption::encrypt;

#[derive(Parser)]
struct Args {

    #[clap(short, long)]
    add: String,

    #[clap(short, long)]
    new: String
}

fn main() {

    // let args = Args::parse();

    // input message
    let message = "Hello, world!";
    let result = encrypt(message);
    println!("result: {}", result);
}
