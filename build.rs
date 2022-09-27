fn main() {
    let mut build = cc::Build::new();
    #[cfg(target_os = "linux")]
    build.define("LINUX", "");

    #[cfg(not(target_os = "linux"))]
    panic!("unsupported target OS");

    build
        .file("src/term.c")
        .compile("c-part.a");
}
