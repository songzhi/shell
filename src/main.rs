use shell::cli::cli;

fn main() {
    if let Err(err) = cli() {
        println!("{}", err);
    }
}
