// Usage: find [options] [path...] [expression]
mod myfind;
use std::process::exit;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if let Err(err) = myfind::run(myfind::Args::new(args)) {
        eprintln!("find: {}", err);
        exit(1);
    }
}
