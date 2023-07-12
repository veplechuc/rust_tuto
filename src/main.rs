fn main() {
    println!("Hello, world!");
    let x = 5; // variables ARE inmutable!!!
    println!("{}", x);

    // x= 10; error because it is inmutabñe

    let x = "six"; // shadowing variable x

    let _variable : i128;  // '_' previene que el compilador diga que no esta usada


    println!("{}", x); // prints 6 and 6%
}

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