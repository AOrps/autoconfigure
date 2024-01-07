use clap::Parser;
use gtmpl;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // Name of the person to greet
    #[arg(short,long)]
    name: String,

    // Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}



fn main() {
    let args = Args::parse();
    for _ in 0..args.count {
	println!("Hello {}!", args.name);
    }


    let output = gtmpl::template("Finally, some template {{ . }} in rust", "nuts");
    // assert_eq!(&output.unwrap(), "Finally, some template nuts in rust");
    println!("Hello, world!");
    println!("{:?}", &output.unwrap());
}
