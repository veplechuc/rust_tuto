#![allow(unused)]
#![allow(clippy::let_unit_value)]

// Closures automatic borrow a reference to in scope variables

fn main() {
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
