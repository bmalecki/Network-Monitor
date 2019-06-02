use std::process;

fn main() {
    if let Err(e) = projekt_zaliczeniowy::run() {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
