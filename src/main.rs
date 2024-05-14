use std::env;

fn main() {
    if env::args().any(|a| a == "--debug") {
        eprintln!("--debug so break");
        return;
    }

    loop {
        eprint!("no debug loop");
        return;
    }
}
