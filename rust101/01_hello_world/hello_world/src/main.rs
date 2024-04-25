fn main() {
    println!("Hello, world!");
}

/*
    Some notes on the above (see https://doc.rust-lang.org/book/ch01-03-hello-cargo.html):
    
    Use `cargo new ${binary_name} to create a new rust directory`.
    Source code is added to the `src` directory.
    Use `cargo build` to build the binary and place it in /target/debug/${binary_name.exe}.
    Use `cargo run` to build the binary and run it immediately.
    Use `cargo check` to make sure the code compiles without actually building anything.
    Use `cargo build --release` to compile without optimisations.
    `cargo build` should work on any rust git repo - just download, build and you should be ready.
*/