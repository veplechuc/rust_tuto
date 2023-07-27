enum IpAdrr {
    V4(u8, u8),
    V6,
}

enum Message {
    Quit,                    //stores no data
    Move { x: i32, y: i32 }, // stores a anonymous struct
    Write(String),           // single string
    ChangeColor(i128),       //store a 128 integer
}

//define an implementation block for the message
impl Message {
    fn algo() {
        println!("some text");
    }
}

// RUST DOES NOT HAVE NULL VALUES
// so we need to wrapped into an option definition
// enum Option<T>{
//     Some(T), // uses when the variable has some vslue
//     None, // if the variable is null
//  }

//matching
//SOME and NONE siempre se deben definir sino usar "_"

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
    #[allow(unused)]
    White, //para que no se queje el compilador si no se usa
}

enum Something {
    Number(i32),
    #[allow(unused)]
    Name(String), //para que no se queje el compilador si no se usa
    Unknown,
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

    let s: Something = Something::Name("some name"); // we need to specify the value
    let s: Something = Something::Number(5); // we need to specify the value
    match s {
        Something::Number(n) => println!("{}", n), // extract the value to a new variable
    }
    let c: Colour = Colour::Red;

    match c {
        Colour::Red => println!("Rojo"),
    }
}
