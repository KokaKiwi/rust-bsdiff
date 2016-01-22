extern crate gcc;

const SOURCES: &'static [&'static str] = &[
    "bsdiff/bsdiff.c",
    "bsdiff/bspatch.c",
];

fn main() {
    // Compile bsdiff4
    let mut config = gcc::Config::new();
    for source in SOURCES {
        config.file(source);
    }
    config.compile("libbsdiff.a");
}
