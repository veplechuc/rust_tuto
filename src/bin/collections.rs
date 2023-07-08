//sequences types 
//-----------------
// Vec -> a growable sequence of elements
// Vedeque -> double ended queue (not always contiguous in memory)
// LinkedList -> doubly-linked list containing a series of nodes

//maps
//-------
// hashmaps -> key - value store
// Btreemap -> map optimized for search

//sets
//--------
// Hashset -> a hashmap but with no duplicates -- only key
// BtreeSet -> betreemap for sets


//misc
//-------
// binary heap -- priority queue implementation


// Arrays -> fixed size, contiguos in memory, objects of the same type are 0 base

// Tuples -> finite (contains different types sequensce of elementes) objs of different types

//Slices -> views into a block of memory, mutable o shared, dynamically sized


use std::collections::HashMap;



fn main() {
    let mut my_vec = vec![1,2,3];

    my_vec.push(9);

    let str_sequence = String::from("Hello");

    let mut coffe =  HashMap::new();
    coffe.insert("latte", 10);

    for (name, value) in &coffe {
        println!("name ->{name}, vlaue->{value}");
    }

    let tup = (1,2, "something");

    let my_array: [u8; 4] = [1,2,6 , 3];

    let mys_slice = &my_array[1..3]; // 2,6,3



}