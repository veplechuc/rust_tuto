#![allow(unused)]
#![allow(clippy::let_unit_value)]

// Closures automatic borrow a reference to in scope variables
// closures  basically is
// let add  = |x, y| {x+y}

use std::vec;

fn main() {
    // Create a closure that defines if someone can vote
    let can_vote = |age: i32| age >= 18;
    println!("Can vote : {}", can_vote(8));

    let s = "🤔";
    let s2 = s.clone();
    let f = || {
        println!("dentro del closure f valor de s --> {}", s); // at this point s get drop
    };

    f();

    // in this case we pass the var takes ownership
    // so we can pass the value to another thread without worry about the
    // scope of s
    let f1 = move || {
        println!("closure move --valor de s --> {}", s);
    };
    println!("despues de f1 valor de s --> {}", s);
    f1();
    println!("valor clonado de s --> {}", s2);

    let sum = |a, b| a + b;
    let prod = |a, b| a * b;

    println!("5 + 4 = {}", use_func(5, 4, sum));
    println!("5 * 4 = {}", use_func(5, 4, prod));

    //concatenate closures
    let add = |n1, n2| n1 + n2;
    let (from, to) = (1, 10);
    // no need to pass the from and to values
    // from an to are inferred from the scope
    let sum_range = || {
        let mut sum = 0;
        for n in from..to {
            sum = add(sum, n);
        }
        sum
    };

    println!("sum = {}", sum_range()); // we call the closure

    //////  a shorter version
    let suma = (1..10).fold(0, |ini, n| ini + n);

    println!("suma = {}", suma);

    // explicitly define the type for resu or put .sum::<i32>()
    let resu: i32 = (1..100).filter(|n| n % 2 == 0).sum();
    println!("odd numbers sum = {}", resu);

    // closures takes the type of the value of the first call
    let double = |num| num * 2;
    let x = 10;
    double(x);

    // to uncomment these lines we need to comment the x definition
    // let y = 10.5;
    // double(y);
    let mut vect = vec![1, 2, 3];
    let mut some_closure = || {
        vect.push(10);
        println!("{:?}", vect)
    };

    //println!("{:?}", vect);

    some_closure();

    // FUNCTION TYPES

    let resp = repeat(add_val, 50);
    println!("value of repeat ->{}", resp);

    ////////////// ITERATOR
    // reads a text file from disc, the odd lines
    // skips 2 lines and takes the consecutives 2
    // Rust "pulls" values from the structure, does not push it

    let file = std::fs::read_to_string("src/texto.txt").unwrap();
    file.lines()
        .enumerate()
        .skip(2)
        .take(4) // this returns a tuple
        .filter(|(idx, _)| idx % 2 == 0)
        .for_each(|(_, line)| println!("{}", line));
}

// You can pass closures to functions
fn use_func<T>(a: i32, b: i32, func: T) -> i32
where
    T: Fn(i32, i32) -> i32,
{
    func(a, b)
}

// FUNCTION TYPES
fn add_val(a: i32) -> i32 {
    a + 10
}
// specifies the type an return value of the func when passed in as param
fn repeat(fun: fn(i32) -> i32, arg: i32) -> i32 {
    fun(arg) + fun(arg)
}
////////////////////////////
fn iter_ator() {
    // ITERATORS -----------------

    let v = vec![2, 3, 4, 5];

    let odd_total = v
        .into_iter() //iter adaptor
        .map(|x| x * 3) // iter adaptor
        .filter(|y| *y % 2 == 0) // iter adaptor
        .for_each(|z| println!(" valor de z --> {}", z));
    // ends of those iterators adapters with an iterator consumer
    // example .sum()
    // or add a "turbofish '::<>' " sum::<i32>() sintax
    // another iter consumer -> collect() put all the items in a new collection
    // v.iter() ... iterate over the collection inmutables references for num in v.iter() - or  for _ in &v and dont change the value
    // v.iter_mut() .. iterate over the collection of mutables references for num in v.iter_mut() or for _ in &mut v and change the value
}
