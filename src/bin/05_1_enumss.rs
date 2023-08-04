#![allow(dead_code)]
enum IpAdrr {
    V4(u8, u8),
    V6,
}

enum Message {
    Quit,                    // stores no data
    Move { x: i32, y: i32 }, // stores a anonymous struct
    Write(String),           // single string
    ChangeColor(i128),       // store a 128 integer
}

// define an implementation block for the message
impl Message {
    fn algo() {
        println!("some text");
    }
}

// MATCHING
// SOME and NONE siempre se deben definir sino usar "_"

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
        // _ => None si queremos matchear cualquier otra cosa
    }
}

// ENUM

enum Colour {
    Red,
    #[allow(unused)] // whit this line, the compiler wont complain if not used
    White,
}

enum Something {
    Number(i32),
    #[allow(unused)] // whit this line, the compiler wont complain if not used
    Name(String),
    Unknown,
}

enum AnotherColor {
    Red,
    Blue,
    Rgb(u8, u8, u8),                    // this is s tuple member
    Invented { cyan: u8, magenta: u8 }, //this is s struct like member
}

// ENUM OPTION
// rust define it on the std library
// enum Option<T>{
//     Some(T),
//     None
// }

fn main() {
    let algo = Some(5);
    let y = 2;

    let sum = algo.unwrap_or(0) + y;

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let s: Something = Something::Name("some name".to_string()); // we need to specify the value
    let s: Something = Something::Number(5); // we need to specify the value
    match s {
        Something::Number(n) => println!("{}", n), // extract the value to a new variable
        _ => println!("Nothing..."),
    }
    let c: Colour = Colour::Red;

    match c {
        Colour::Red => println!("Rojo"),
        _ => println!("no taken into account"),
    }

    let another: AnotherColor = AnotherColor::Invented {
        cyan: 10,
        magenta: 20,
    };

    match another {
        //     AnotherColor::Red => println!("Rojo"),
        //     AnotherColor::RGB(, , )
        AnotherColor::Invented {
            cyan: _,
            magenta: 122,
        } => println!("invented"),

        _ => println!("no taken into account"),
    }
}
