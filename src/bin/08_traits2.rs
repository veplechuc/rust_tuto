#![allow(unused)]

struct Dog {
    name: String,
}

struct Cat {
    name: String,
}

struct Ant {
    name: String,
}

struct Spider {
    name: String,
}

//THINK ON TRAITS AS A SOME KIND OF INTERFACE
// the method can implemented or not if it is implemented
// the specific implementation of the struct
// can use it as default. check Ant or Spider impl
trait Animal {
    // fn talk(&self) -> String; // &self, allows the method th acces the data in the implemented strcut
    fn talk(&self) -> String {
        String::from("Cant talk!!")
    }
}

impl Animal for Dog {
    fn talk(&self) -> String {
        String::from("DOg  makes sounds -- from Dog impl")
    }
}

// define a DEFAULT IMPLE FOR THOSE THAT HAVE SAME CODE
impl Animal for Ant {}

impl Animal for Spider {}

// TRAITS AS PARAMETERS AND TRAITS BOUNDS

fn make_animal_talk(animal: &(impl Animal)) {
    // usamos 2 params en trits + una funcion ahora make_animal_talk acepta cualquier type que
    // implemente Animal traits ... en lugar de la impl concreta
    // como definimos animal como impl Animal + Display podemos llamar a talk e imprimir
    println!(
        "imprimimos lo correspondiente implementado en rl trait {}",
        animal.talk()
    );
}
// USANDO GENERICS es lo mismo que lo anterior
fn make_generic_talk<T>(animal: &T)
where
    T: Animal,
{
    println!("imprimimos usando generics {}", animal.talk());
}

/*
podemos usar multiples TRAITS
fn make_generic_talk <T>(animal: &T)
where T: Animal + std::fmt::Display,{ <<---- com usamos aca
    println!("imprimimos usando generics {}", animal.talk());
}
o bien
fn make_animal_talk(animal: &(impl Animal + std::fmt::Display)){
    ....
}

*/

// Retuning values from TRAITS
// BOX ALLOWS MEMORY ALLOCATION ON THE HEAP

fn ret_val() -> impl Animal {
    Dog {
        name: "from_retunr".to_string(),
    }
}

fn ret_val_box(var: bool) -> Box<dyn Animal> {
    if var {
        Box::new(Dog {
            name: "Box_terri".to_string(),
        })
    } else {
        Box::new(Spider {
            name: "man".to_string(),
        })
    }
}

// ASSOCIATED TYPES
trait Incrementor {
    type Item; //In traits, type is used to declare an associated type
    fn increment(&mut self) -> Self::Item; // returns the valeu of that type
}

struct Counter {
    count: u32,
}

impl Incrementor for Counter {
    type Item = u32;
    fn increment(&mut self) -> Self::Item {
        self.count += 1;
        self.count
    }
}

// RULE:
// FOR MULTIPLE CONCRETE TYPES -->> USE GENERICS
// ONLY FOR ONE TYPE -->> USE ASSOCIATED TYPES
// RUST allows  trait to have a method that shares the same name as a method from another trait
// it's also possible to implement a method directly on a type, with the same name as a method from another trait.

trait File {
    fn write(&self);
}

struct Logger;
impl Logger {
    fn write(&self) {
        println!("write this from logger to screen...")
    }
}

impl File for Logger {
    fn write(&self) {
        println!("write this from File to screen...")
    }
}

// SUPER TRAITS
trait Speak {
    fn speack(&self);
}

trait Greet: Speak {
    fn greet(&self) {
        self.speack();
        println!(" esto es especifico de geet...");
    }
}

struct Person {
    name: String,
}

impl Speak for Person {
    fn speack(&self) {
        println!("Hola desde Person strut imp..{}", self.name);
    }
}

use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Coordinate {
    // with this we can pass an I32 or f64
    longitude: f64,
    latitude: f64,
}

impl Add for Coordinate {
    type Output = Coordinate;

    fn add(self, other: Coordinate) -> Coordinate {
        //rhs = right hand side
        Coordinate {
            longitude: (self.longitude + other.longitude),
            latitude: (self.latitude + other.latitude),
        }
    }
}

fn main() {
    let dog = Dog {
        name: "NAME".to_string(),
    };

    let ant = Ant {
        name: "Ant NAME".to_string(),
    };

    let spider = Spider {
        name: "Spider NAME".to_string(),
    };

    // println!("{} -->> {}",dog.name, dog.talk());
    // println!("{} -->> {}",ant.name, ant.talk());
    // println!("{} -->> {}",spider.name, spider.talk());
    // make_animal_talk(&dog);
    // make_generic_talk(&spider);
    // // reutning values
    let animal = ret_val();
    println!("from ret_val {}", animal.talk());

    let dog = ret_val_box(true);

    // associated types
    let mut couter = Counter { count: 0 };
    // for _ in 0..10 {
    //     println!("Counter: {}", couter.increment());
    // }

    // let log = Logger; // callls the write method from logger
    // log.write();
    // si necesitamos el log especifico de File usamos
    // File::write(&log);

    assert_eq!(
        Coordinate {
            latitude: 1.0,
            longitude: 2.0
        } + Coordinate {
            latitude: 3.0,
            longitude: 4.0
        },
        Coordinate {
            longitude: 6.0,
            latitude: 4.0
        }
    )
    //SUPERTRAITS ...
}
