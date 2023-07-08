enum IpAdrr {
    V4(u8, u8),
    V6
}

enum Message {
    Quit, //stores no data
    Move {x:i32, y:i32}, // stores a anonymous struct
    Write(String), // single string
    ChangeColor (i128) //store a 128 integer 
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

fn plus_one(x:Option<i32>) -> Option<i32>{
    match x {
        None => None,
        Some(i) => Some(i +1),
        // _ => None si queremos matchear cualquier otra cosa

    }
}

fn main(){

    let algo = Some(5);
    let y = 2;


    let sum = algo.unwrap_or(0) + y;

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}