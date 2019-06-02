extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/rtnetlink.c")
        .compile("rtnetlink");
}