#![allow(dead_code)]
// struts is used to group values/characteristics

//Composition:
//Instead of using inheritance, Rust encourages the use of composition.
//Composition means that one struct can include another struct as a field
struct Parent {
    // Parent fields
}

struct Child {
    parent: Parent,
    // Child-specific fields
}

impl Parent {
    // Parent methods
}

impl Child {
    // Child-specific methods
}

///////////////////// example of default
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
    grade: String,
}
impl Default for Person {
    fn default() -> Person {
        Person {
            name: "Alfred".to_string(),
            age: 0,
            grade: "Junior".to_string(),
        }
    }
}

////////////////////////////////////////////

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

struct Product {
    name: String,
    price: f32,
    in_stock: bool,
}

/* a struct could have many impl defined */
impl Product {
    fn new(name: String, price: f32) -> Product {
        Product {
            name: name,
            price: price,
            in_stock: true,
        }
    }
    // we can use Self
    // fn new(name: String, price: f32) -> Self {
    //     Self { ...

    fn get_default_sales_tax() -> f32 {
        0.1
    }
    // this is a METHOD associated to the implementation instance, because uses "self"
    fn calculate_sales_tax(&self) -> f32 {
        self.price * Product::get_default_sales_tax()
    }

    fn set_price(&mut self, price: f32) {
        self.price = price;
    }

    fn buy(self) -> f32 {
        let name = self.name;
        println!("{} was bought! for {}", name, self.price);
        self.price
    }
}

/* Strutcs can be taken as the definition of object attributes on OOP
allows to define a new type that acces by name the attribute

*/
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 2,
    }
}

fn set_email(u: User) -> User {
    User {
        email: String::from("contact@im.dev"),
        ..u
    }
}

fn main() {
    let tmp_all_default = Person {
        ..Person::default()
    };
    let tmp_with_name = Person {
        name: "Sam".to_string(),
        grade: "Master".to_string(),
        ..Person::default()
    };

    println!("tmp_all_defaul {:#?}", tmp_all_default);
    println!("tmp_with_name {:?}", tmp_with_name);

    // create a new intance
    let u1 = User {
        email: String::from("someone@example.com"),
        username: String::from("sunface"),
        active: true,
        sign_in_count: 1,
    };

    let u2 = set_email(u1);
    println!("{:?}", u2);
    println!("Success!");

    // OTHER STRUCT DEF
    struct Colour(i32, i32, i32);

    struct Point(i32, i32, i32);
    /*
    you can use strcut to group data but you can not pass
    to a func expecting a Colour struct a Point struct
    */

    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale), // Print debug info to stderr and assign the value of  `30 * scale` to `width`
        height: 50,
    };

    dbg!(&rect1); // Print debug info to stderr

    println!("{:?}", rect1); // Print debug info to stdout

    //////////////////////////////////////////

    let mut book = Product::new(String::from("Book"), 30.0);
    let sales_tax = book.calculate_sales_tax();
    println!("sales tax: {}", sales_tax);
    book.set_price(1.0);
    // we can use the Product struct an pass in the ref to the instance
    Product::set_price(&mut book, 9910.3);

    book.buy();

    #[derive(Debug)]
    struct Person2 {
        name: String,
        age: Box<u8>,
    }

    let person = Person2 {
        name: String::from("Alice"),
        age: Box::new(20),
    };

    // `name` is moved out of person, but `age` is referenced
    let Person2 { name, ref age } = person;

    println!("The person's age is {}", age);

    println!("The person's name is {}", name);

    // Error! borrow of partially moved value: `person` partial move occurs
    //println!("The person struct is {:?}", person);

    // `person` cannot be used but `person.age` can be used as it is not moved
    println!(
        "The person's age from person struct is {}. {}",
        person.age, name
    );
}
