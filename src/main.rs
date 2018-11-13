use std::env;

mod yaml;

fn main() {

    let args: Vec<_> = env::args().collect();
    if args.len() == 1 {
        println!("No arguments given, exiting...");
        std::process::exit(1);
    }
    let docs = yaml::from_file(&args[1]);
    println!("{:?}", docs);
    println!("\n\n{}", docs[0][2]["key"].as_str().unwrap());

}
