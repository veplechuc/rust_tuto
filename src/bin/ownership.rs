// Owner ship rules

/*
1 Each value en Rust has a varible that's called its owner
2 There can only be one owner at a time
3 When the owner goes out of the scope, the value will be dropped
 */

/*MEMORY options

Garbage collector PROS error free, facter write time, CONS: No memory control, slower unpredictable runtime performance, larger programs
MAnual Management: PROS: control over memory, faster runtime, smaller program, CONS error prone, slower write time

Rust has the ownership model on memory managment
Pros
-control over memory
-error free
-faster runtime
-smaller program
CONS
-slower write time - Learning curve (borrow checker)
 */

fn main() {
    let x = 5;
    let y = x; // here is a copy happening BECAUSE rust implements the TRAITS copy  AND WORKS WITH INT, BOOL, CHAR

    let s1 = String::from("some text");
    // let s2 = s1; // it move the value NOT shallow copy

    //        println!("{}", s1);
    // |                    ^^ value borrowed here after move
    // println!("{}", s1);  <-- error
    //TO COPY SOME VALUE THAT IT IS STORE IN THE HEAP

    let s2 = s1.clone();

    // OWNERSHIP IN FUCTIONS
    let s = String::from("some text");
    takes_ownership(s); // here we are passing in the ownership be careful!!!
                        //println!("{}", s); CAN NOT DO THIS here s does not exist enymore

    //second example
    let s_ref = String::from("some another text");
    let len: usize = from_reference(&s_ref); // el & nos permite pasar la referencia y no perder s
    println!("the length of {} is {}", s_ref, len);

    //third example
    let mut s_mut = String::from("some another text");
    from_mut(&mut s_mut); // el & nos permite pasar la referencia y no perder s
    println!("S_MUT  is {}", s_mut);
}

// esta funcion toma el ownership de la variable s
fn takes_ownership(some_str: String) {
    println!("{}", some_str);
}
// second example
fn from_reference(some_var: &String) -> usize {
    // esta funcion hace un borrow de s pero no se puede cambiar s
    let length: usize = some_var.len();
    return length;
}

// esta funcion toma la referencia de una var mutable y le agrega datos
fn from_mut(some_var: &mut String) {
    some_var.push_str(" agregado");
}

//NO SE PUEDE TENER LA REFERENCIA DE UNA MUTABLE MAS DE UNA VEZ
// SI SE PUEDE TENER MAS DE UNA REFERENCIA A UNA VAR INMUTABLE
fn double_ref() {
    let mut s_mut = String::from("some another text");

    let _r1: &mut String = &mut s_mut;
    //let _r2: &mut String = &mut s_mut; cannot borrow `s_mut` as mutable more than once at a time

    // println!("{} {}", _r1, _r2)

    let no_mut = String::from("some another text");

    let _r1: &String = &no_mut;
    let _r2: &String = &no_mut;

    /// COSNIDERAR SIEMPRE EL SCOPE PORQUE COMIENZA CUANDO SE DECLARA Y FINALIZA CUANDO SE USA POR ULTIMA VEZ
    let mut ns_mut = String::from("some another text");

    let _r11: &String = &ns_mut; // R11 SCOPE COMIENZA ACA
    let _r21: &String = &ns_mut;

    println!("{} {}", _r11, _r21); //R11 SCOPE TERMINA ACA _(SI NO SE VULEVE A USAR)

    let _r31: &mut String = &mut ns_mut; //ESTO ANDA PORQUE R11 Y R21 YA NO ESTAN EN EL SCOPE
}

fn call_dangle() {
    let refer_to_nothing = dangle();
}

fn dangle() -> &String {
    //<<<--- CHECK FOR THE ERROR THAT HAPPENS HERE

    let s: String = String::from("value");
    &s
}

// REFERENCES rules

// 1 AT ANY GIVEN TIME YOU CAN HAVE EITHER ONE MUTABLE REFERENCEOR ANY IMMUTABLE REFERENCES
// 2 REFERENCES MUST ALWAYS BE VALID

// SLICING ...

fn slicing() {
    let mut s: String = String::from("value valu3");

    let val = &s[..5]; // [0..5] if it is from the begining can ommit the begining
    let val2 = &s[6..]; // [6..11]if it until the end can ommit the end value
    let val3 = &s[..]; // all the slicing
}
