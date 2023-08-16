use rust_tuto::Puzzle;

mod deeply {
    pub mod nested {
        pub fn function() {
            println!("called `deeply::nested::function()`");
        }
    }
}

fn main() {
    // Easier access to `deeply::nested::function`
    ext_mod::function();

    println!("Entering block");
}
