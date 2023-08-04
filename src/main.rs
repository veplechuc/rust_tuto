fn main() {
    println!("Hello, world!");
    let x = 5; // variables ARE inmutable!!!
    println!("{}", x);

    // x= 10; error because it is inmutable

    let x = "six"; // shadowing variable x

    let _variable: i128; // '_' prevent the compiler complains about unused variable

    println!("{}", x); // prints 6 and 6%
}

// MANAGING RUST VERSIONS

// A particular version of Rust (and it's associated version of the standard library and other tools) is called a toolchain.
// rustup can manage multiple toolchains on your machine at once. https://rust-lang.github.io/rustup/concepts/toolchains.html
// Then, you can use directory overrides (https://rust-lang.github.io/rustup/overrides.html#directory-overrides)
// to customize which versions of Rust you use locally,
// or you can add a toolchain file to specify that no matter which machine a project is on,
// it must use a specific version of Rust (https://rust-lang.github.io/rustup/overrides.html#the-toolchain-file)

// MAIN STRUCTURE OF RUST PROJECT
// .
// ├── Cargo.lock
// ├── Cargo.toml
// ├── src/
// │   ├── lib.rs
// │   ├── main.rs
// │   └── bin/
// │       ├── named-executable.rs
// │       ├── another-executable.rs
// │       └── multi-file-executable/
// │           ├── main.rs
// │           └── some_module.rs
// ├── benches/                         this is for benchmarking propouse
// │   ├── large-input.rs
// │   └── multi-file-bench/
// │       ├── main.rs
// │       └── bench_module.rs
// ├── examples/
// │   ├── simple.rs
// │   └── multi-file-example/
// │       ├── main.rs
// │       └── ex_module.rs
// └── tests/
//     ├── some-integration-tests.rs
//     └── multi-file-test/
//         ├── main.rs
//         └── test_module.rs
