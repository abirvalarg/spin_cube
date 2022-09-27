fn main() {
    cc::Build::new()
        .file("src/term.c")
        .compile("c-part.a");
}
