#![allow(dead_code)]
// Traits can be implemented by
// struct, closure, enum function

// Adding #[derive(Clone, Debug)] applies those traits to the specific struct
#[derive(Clone, Debug)]
pub struct Algo {
    var1: u32,
    var2: u32,
}
#[derive(Clone, Debug)]
pub struct Otra {
    var11: u32,
    var12: u32,
}
pub trait Somthing {
    // can be taked as a abstrac or interface for structs
    fn una_func(&self);
}

impl Somthing for Algo {
    fn una_func(&self) {
        println!("implementation of una_func for Algo struct \n")
    }
}

impl Somthing for Otra {
    fn una_func(&self) {
        println!("implementation of una_func for Otra struct \n")
    }
}

// if we want to goup them we can use a vector

pub struct All {
    pub components: Vec<Box<dyn Somthing>>,
}

impl All {
    pub fn print_all(&self) {
        for component in self.components.iter() {
            println!("Inside print all func Impl..");
            component.una_func();
        }
    }
}

// implementation of the Default trait for Algo (use VS code to help)
impl Default for Algo {
    fn default() -> Self {
        Algo { var1: 99, var2: 99 }
    }
}

// we can use #[derive(Default)].. but gives 0 numbers empty strings and collections

fn main() {
    let all_ptr: All = All {
        components: vec![
            Box::new(Algo { var1: 12, var2: 15 }),
            Box::new(Otra {
                var11: 45,
                var12: 88,
            }),
        ],
    };
    all_ptr.print_all();

    let algo = Algo { var1: 10, var2: 1 };

    let algo_default: Algo = Default::default();

    algo.una_func();

    // since we implement derive Clone
    let al = algo.clone();

    //her we use the Default implemented Trait
    let algo_default = Algo {
        var1: 100,
        ..Default::default()
    };
}
