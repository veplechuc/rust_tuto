use rust_tuto::file_mod1 as pprt;
use rust_tuto::maths::basic_m::multi;

use rust_tuto::my_mod::my_mod1::function;

fn main() {
    // Easier access to `deeply::nested::function`
    // ext_mod::function();
    pprt::printin();
    let n1 = 10;
    let n2 = 58;
    println!("{}", multi(&n1, &n2));
    println!("Entering block");
}
