pub use kvs::cli;
use std::process::exit;

fn main() {
    let matches = cli::cli();
    match matches.subcommand() {
        ("set", Some(_matches)) => {
            eprintln!("unimplemented");
            exit(-1);
        }

        ("get", Some(_matches)) => {
            eprintln!("unimplemented");
            exit(-1);
        }

        ("rm", Some(_matches)) => {
            eprintln!("unimplemented");
            exit(-1);
        }

        (_, _) => unreachable!(),
    }
}
