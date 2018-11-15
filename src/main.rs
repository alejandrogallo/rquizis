extern crate clap;

use clap::{Arg, App};

mod ui;
mod yaml;

fn main() {
    let matches = App::new("rquizis")
        .version("0.1.0")
        .arg(
            Arg::with_name("file")
            .takes_value(true)
        )
        .get_matches();

    let input_file = matches.value_of("file").unwrap().to_string();
    let docs = yaml::from_file(&input_file);
    println!("{:?}", docs);
    println!("\n\n{}", docs[0][2]["key"].as_str().unwrap());

    ui::tui::main();

}
