use std::{num::ParseIntError, vec};

fn to_int(s: &str) -> i32 {
    //s.parse().unwrap() // parse retorna un -> Result<F, F::Err>
    // Result tiene una funcion -> unwrap que si resulto retorna un ok
    // devuelve ese valor sino, deja que el programa entre en Panic y falle

    // ---------------------
    //variant
    //s.parse().expect("algo paso con el parseo") // deja enviar un mensaje cuando pasa algun error
    // ---------------------
    //
    s.parse().unwrap_or(0) //Returns the contained [Ok] value or a provided default.
}

fn to_int_op(s: &str) -> Option<i32> {
    s.parse().ok()
}

fn sum(strs: Vec<String>) -> String {
    let mut ac = 0;
    for s in strs {
        ac += to_int(&s);
    }
    ac.to_string()
}

fn sum_option(strs: Vec<String>) -> String {
    let mut ac = 0;
    for s in strs {
        // USANDO PATTER MATCHING
        // ac += match to_int_op(&s) {
        //     Some(val) => val,
        //     None => 0,
        // }
        //USANDO IF LET
        // if let Some(val) = to_int_op(&s) {
        //     ac += val
        // }
        // USANDO UNWRAP OR
        ac += to_int_op(&s).unwrap_or(0);
    }
    ac.to_string()
}

fn sum_option2(strs: Vec<String>) -> Option<String> {
    // esta func retorna una Option
    let mut ac = 0;
    for s in strs {
        ac += to_int_op(&s)?; // ? -> propaga el None en el stack
    }
    Some(ac.to_string())
}
#[derive(Debug)]
struct SumError;

// que pasa si queremos ahacer algo con el error que se pueda generar
fn sum_option3(strs: Vec<String>) -> Result<String, SumError> {
    // DEVUELVO SI ESTA BIN P ERROR
    // esta func retorna una Result
    let mut ac = 0;
    for s in strs {
        ac += to_int_op(&s).ok_or(SumError)?; // propaga el None en el stack
    }
    Ok(ac.to_string())
}

fn to_int_resu(s: &str) -> Result<i32, ParseIntError> {
    s.parse()
}

// que pasa si queremos ahacer algo con el error que se pueda generar
fn sum_option4(strs: Vec<String>) -> Result<String, SumError> {
    // DEVUELVO SI ESTA BIN P ERROR
    // esta func retorna una Result
    let mut ac = 0;
    for s in strs {
        ac += to_int_resu(&s).map_err(|_| SumError)?;
    }
    Ok(ac.to_string())
}

fn main() {
    let v = vec![String::from("3"), String::from("4")];
    let total = sum(v);
    println!("resultado suma {:?}", total);

    let v = vec![String::from("3"), String::from("ab4")];
    let total = sum(v);
    println!("resultado otra suma {:?}", total);

    let v = vec![String::from("3"), String::from("ab4")];
    let total = sum_option2(v);
    println!("resultado otra suma {:?}", total); // aca esperamos obtener NONE
}

//contenedore

pub enum Option<T> {
    None,    // struct sin fields
    Some(T), // es una tuple struct
}

pub enum Result<T, E> {
    // explicit defines success and error states
    /// Contains the success value
    Ok(T), // tuple struct
    /// Contains the error value
    Err(E), // tuple struct
}

//unwrap toma el valor del container y lo devuelve si existe sino se puede especificar que hacer si no se hace nada panic!

// alg.ok convierte entre Result y Options

//aalgo.map convirte los tipos de los contenedores ejemplo un Result conun error type pasarlo a otro
// result con otro error type

// ? operator, propaga la ausencia de un valor o el success en el caso de option los tipos de error no necesitan ser iguales
// para Result si
