fn main() {
    autocxx_build::Builder::new("src/lib.rs", ["src"])
        .extra_clang_args(&["-std=c++20"])
        .build()
        .unwrap()
        .flag("/std:c++20")
        .compile("repro");
}
