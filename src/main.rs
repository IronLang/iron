extern crate clap;

mod module;

use clap::{App, Arg};
use module::Module;
use std::str::FromStr;

fn main() {
    let matches = App::new("ironc")
        .version("0.1")
        .about("Compiles an Iron module.")
        .arg(
            Arg::with_name("INPUT")
                .help("Path to file containing Iron source code")
                .required(true),
        )
        .get_matches();

    let path = matches.value_of("INPUT").expect("expected INPUT");
    let module = Module::from_str(path);

    println!("{:#?}", module);
}
