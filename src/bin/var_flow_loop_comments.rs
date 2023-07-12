#![allow(unused)]

use rand::Rng;
use std::{io, result};


//RUST CORE DESIGN PHILOSOPHIES 
// Safety --var used in does not change so no bugs
// concurrency --  shared on different threads  - 
// speed -- the compiler know the value have optimized the machine code produced 

// -compiled
// - strongly typed
// - secure
// -no garbage collector
// - no implicit casting
// - variables are inmutable by defaul


fn main() {
    println!("Hello, world!");
    let x = 5; // variables ARE inmutable!!!
    println!("{}", x);

    // x= 10; error because it is inmutabñe

    let x = "six"; // shadowing variable x

    let _variable : i128;  // '_' previene que el compilador diga que no esta usada


    println!("{}", x); // prints 6 and 6%

    const DATA: i32 = 600_000; // const MUST be annotated can not be mutable

    let dat: u8 = 255; //when it reaches the maximun values then 256 -> 0, 257 -> 1 .....


     println!("{}", dat);

    // compound types
    let tup = ("some tuple", 120);
    println!("{}", tup.0);
    let (tup1, tup2) = tup;

    let assigned_by_adot = tup.1;

    println!("tup.0 -> {}--- tup1->{} tup2->{}", tup.0, tup1, tup2);
    // array

    let arr = [1, 2, 3];

    let arr1 = [1; 5]; // [1,1,1,1,1,]
    for x in arr1 {
        print!("{}", x);
    }



    // FUNCTIONS snake case name
    let sum = suma(22, 55);
    println!("resultado de la suma {}", sum);
    println!("resultado de la suma {}", suma(55, 77));

    // control flow
    if sum < 10 { // condition must be explicity a boolean -> ex:  if number {} error
        println!("sum es menos que 10"); // expresion must evaluate to boolean   
    } else if sum > 900 {
        println!("sum es mayor que 900");
    }else {
        println!("sum esta entre 10 y 900");
    }

    //ternary
    let condition = true;
    let numb = if condition {7} else {10};

    //loop
    loop {
        println!("inside loop");
        break;
    }

    // break uses conditional names to breaks
    'bob: loop {
        loop {
            loop {
                break 'bob;
            }
        }
    }

    let mut coun = 0;

    let result = loop {
        coun +=1;
        if coun  == 10 {
            break coun;
        }
    };
    println!("valor de coun {}", coun);
    println!("valor de coun {}", result);

    let ele =[1,2,3,4,5];

    println!("using loop with iter ");
    for el in ele.iter() {
        println!("{}", el);
        
    };

    println!("using loop with for.. in .. ");
    for n in 1..10 { // last number is exclusive
        println!("{}", n);
    };

    println!("using loop while..");
    while num != 0  {
        num = num -1;
        println!("{}", num);
    };

    /* multiline comment
        this is a multiline
        comment
     */
    
    
    //labeld loop;

    'outer: loop {
        
        loop {
            break 'outer;
        };
    };
    
}

fn suma(x: i32, y: i32) -> i32 {
    println!("algo{} ", x);
    return x + y; // o sin return la ultima sentencia
}


// formatting code run cargo fmt 
// use rustfmt name_file.rs

// rust dont throws exceptions!!!

// andyo@olsensoft.com

// portability use i32 or i16 let x: i32
// for velocity use isize -- let x: isize
// {:>10} pone en un campo de 10 espacios alineados a la derecha
// {:<10} idem pero a la izquierda

println!("{:<10}");
print!("{:*>10}");


// si definimos una func con salida de info hay que retornarla
// fn suma(x: i32, y: i32) -> i32 {....} tiene que devolver algo
// fn suma(x: i32, y: i32) {....} no necesariamente
        // let num = 100;
        // match num{
        //     100 => algo,
        //     25..=50 => lll,
        //     25 | 50 => { algo;
        //                 algo mas }
        //     num if num > 0 => kkkkk,
        //     ...
        //     _   => otra cosa
        // }

// Looping


labeld loop


// ENUM

enum Colour {
    Red, 
    #[allow(unused)]White //para que no se queje el compilador si no se usa
}
let c: Colour = Colour::Red;

match c{
    Colour::Red => println!("Rojo")
}

enum Something {
    Number(i32), 
    #[allow(unused)]Name(String), //para que no se queje el compilador si no se usa
    Unknown
}

let s: Something = Something::Name("some name"); // we need to specify the value
let s: Something = Something::Number(5); // we need to specify the value
match s{
    Something::Number(n) => println! ("{}", n) // extract the value to a new variable
}


// ENUM OPTION
// rust define it on the std library
// enum Option<T>{
//     Some(T),
//     None
// }


// ARRAYs TUPLES
let mut lista = [1,2,3]; // use mut to change the values of the array

let mut vec= Vec::<i32>::new();
let mut vec2 = vec![lista de valores];


vec.push(100);

vec.pop... or vec[0] .. vec.len()

let op = vec.get(index)
match {
    some(value) => println...
    None => {}
}

// OWnership

// String is mutable most types does not have Copy trait dont copy the value it moves
let mut str = String::from("string");
str.push_str("...another") // str = "string...another"

let x = 2;
let y = x; 

let str = String::from("string");
let tr = str; // MOVES THE VALUE STR CAN NOT BE USED ANYMORE!!!

let str = String::from("another");
let tr = str.clone(); // need to clone it order to have a copy


/// BORRROWING

//implicit
let s = String::from('hola');
let st = &s; // borrows but does not owns s

// explicit
let s = String::from('hola');
let st: &String = &s; // borrows but does not owns s

//borrow it is like  when passed an objec to a function as reference

let s = String::from('hola');
let st = &mut s; // borrows but does not owns s
// st is mutable
r.push_str("fasdfas"); // --> hola fasdfas

// FUNCTIONS
//passing variables by reference borrowing so you dont loose the object

fn da(){
    let s = String::from("hola");
    
    f2(&s)
}

fn f2(param: &String){
    ...
}

// MUTABLE
fn da(){
    let mut s = String::from("hola");
    
    f2(&mut s)
}

fn f2(param: &mut String){
    ...
}





