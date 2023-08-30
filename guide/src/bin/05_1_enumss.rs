#![allow(dead_code)]

enum IpAdrr {
    V4(u8, u8, u8, u8),
    V6,
}

enum Message {
    Quit,                    // stores no data
    Move { x: i32, y: i32 }, // stores a anonymous struct
    Write(String),           // single string
    ChangeColor(i128),       // store a 128 integer
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

// enum Result<T, E> {
//    Ok(T),
//    Err(E),
// }

// Example of enums with struct
#[derive(Debug)]
struct Custom {
    age: usize,
    name: String,
}
#[derive(Debug)]
enum Item {
    Number(usize),
    Mycustom(Custom),
}

fn app_item(v: &mut Vec<Item>) {
    v.push(Item::Number(8));
    v.push(Item::Mycustom(Custom {
        age: 22,
        name: "hola".to_string(),
    }));
}

fn main() {
    println!("//////////////////////////////");
    let mut vec: Vec<Item> = vec![];
    app_item(&mut vec);
    println!("{:?}", vec.get(1));
    println!("{:?}", vec[0]);
    println!("//////////////////////////////");

    let s: Something = Something::Name("some name".to_string()); // we need to specify the value
    let s: Something = Something::Number(5); // we need to specify the value
    match s {
        Something::Name(n) => println!("el nombre es ->{}", n), // extract the value to a new variable
        Something::Number(n) => println!("el numero es ->{}", n), // extract the value to a new variable
        _ => println!("Nothing..."),
    }
    let c: Colour = Colour::Red;

    match c {
        Colour::Red => println!("el color es Rojo"),
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

    // MORE EXAMPLES
    // consecutuves values..
    enum Vals {
        Var1, // 0
        Var2 = 10000,
        Var3, // 10001
    }

    println!("var1: {}", Vals::Var1 as u32);
    println!("var2: {}", Vals::Var2 as u32);
    println!("var3: {}", Vals::Var3 as u32);

    // Create an Enum for days of week
    enum Day {
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday,
    }

    // Define function for Day enum
    impl Day {
        fn is_weekend(&self) -> bool {
            match self {
                Day::Saturday | Day::Sunday => true,
                _ => false,
            }
        }
    }

    // Use enum to store todays day
    let today: Day = Day::Monday;

    // Perform different actions based on day
    match today {
        Day::Monday => println!("Everyone hates Monday"),
        Day::Tuesday => println!("Donut day"),
        Day::Wednesday => println!("Hump day"),
        Day::Thursday => println!("Hung out day"),
        Day::Friday => println!("Almost Weekend"),
        Day::Saturday | Day::Sunday => println!("Weekend!!!"),
    }

    // Check if today is a weekend
    println!("Is today the weekend {}", today.is_weekend());
}
