# {{project-name}}

This crate provides an idiomatic Rust API for the [Redis Modules API](https://redis.io/topics/modules-intro).
It allows writing Redis modules in Rust, without needing to use raw pointers or unsafe code. See [here](https://docs.rs/redis-module/latest) for the most recent API documentation.

# Running the module

1. [Install Rust](https://www.rust-lang.org/tools/install)
2. [Install Redis](https://redis.io/download), most likely using your favorite package manager (Homebrew on Mac, APT or YUM on Linux)
3. Run `cargo build`
4. Start a redis server with the `hello` module
    * Linux: `redis-server --loadmodule ./target/debug/examples/lib{{crate_name}}.so`
    * Mac: `redis-server --loadmodule ./target/debug/examples/lib{{crate_name}}.dylib`
5. Open a Redis CLI, and run `HELLO.MUL 31 11`.