use clap::Parser;

#[derive(Parser,Default,Debug)]
struct Arguments {
    ip: String,

    // ip: Option<String>,
    // #[clap(short, long)]
    // config: bool
    // #[clap(short)]
    // full_name: Option<String>,
}

fn main() {
    let args = Arguments::parse();
    // println!("{:?}", args);
    if args.ip == "config" {
        println!("{:?}", "helooo".to_string());
        std::process::exit(0)
    }
    println!("{:?}", args.ip);
    // let i = Arguments::parse(ip);
}

