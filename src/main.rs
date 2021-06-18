mod cli;

fn main() {
    if let Err(err) = cli::App::run() {
        eprintln!("Error:\n  {}", err);
    }
}
