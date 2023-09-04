#![allow(dead_code)]
#![allow(unused_variables)]

// Traits can be implemented by
// struct, closure, enum function
// traits allows to separate the data structure from data behaviour

// REMEMBER:
// we can use the traits methods if we imports the traits itself

use guide::structs_def::{Algo, Otra}; // we imported in order to use it

pub trait Something {
    // can be taken as an interface for structs
    fn una_func(&self);
}

impl Something for Algo {
    fn una_func(&self) {
        println!("implementation of una_func for Algo struct \n")
    }
}

impl Something for Otra {
    fn una_func(&self) {
        println!("implementation of una_func for Otra struct \n")
    }
}

// if we want to goup them we can use a vector

pub struct All {
    pub components: Vec<Box<dyn Something>>,
}

impl All {
    pub fn print_all(&self) {
        for component in self.components.iter() {
            println!("Inside print all func Impl..");
            component.una_func();
        }
    }
}

//

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
