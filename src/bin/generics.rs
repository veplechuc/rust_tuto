// we need to tell the compiler that T implements Display in order to be printed

fn print_data<T: std::fmt::Display>(vec: &[T], label: &str) {
    //for multuple generic params use <T, U, V,...
    println!("{}", label);
    for elem in vec {
        println!("{}", elem);
    }
}
//Another way is
fn print_data2<T>(vec: &[T], label: &str)
where
    T: std::fmt::Display + PartialOrd,
{
    //for multuple generic params use <T, U, V,...
    println!("{}", label);
    for elem in vec {
        println!("{}", elem);
    }
}

//GENERICS WITH STRUCTS

struct Coordinate<T, U> {
    // with this we can pass an I32 or f64
    longitude: T,
    latitude: U,
}

//GENERICS WITH MERHIDS

impl<T: std::fmt::Display, U: std::fmt::Display> Coordinate<T, U> {
    fn display(&self) {
        println!(
            "location latitude: {}, longitude: {}",
            &self.latitude, &self.longitude
        );
    }
}

impl Coordinate<f64, f64> {
    fn dist(&self, other: &Coordinate<f64, f64>) -> f64 {
        let long = &self.longitude + &other.longitude;
        let lati = &self.latitude + &other.latitude;
        long + lati
    }
}

// GENERICS ON ENUMS
enum Result<T, E> {
    Ok(T),
    Err(E),
}

fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Result::Err(String::from("error division by zero"))
    } else {
        Result::Ok(a / b)
    }
}

fn main() {
    let location1 = Coordinate {
        latitude: 3.100,
        longitude: 18.889,
    };

    let location2 = Coordinate {
        latitude: 3.100,
        longitude: 5,
    };

    let location3 = Coordinate {
        latitude: 3.100,
        longitude: 5.558,
    };

    // location1.display();
    // location2.display();
    println!("total {}", location1.dist(&location3));
    let result = divide(5.6, 3.4);
    match result {
        Result::Err(err) => println!("Error: {}", err),
        Result::Ok(res) => println!("Error: {}", res),
    }
}
