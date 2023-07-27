#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
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

impl Product {
    fn new(name: String, price: f32) -> Product {
        Product {
            name: name,
            price: price,
            in_stock: true,
        }
    }
    fn get_default_sales_tax() -> f32 {
        0.1
    }
    fn calculate_sales_tax(&self) -> f32 {
        self.price * Product::get_default_sales_tax()
    }
    fn set_price(&mut self, price: f32) {
        self.price = price;
    }
    fn buy(self) -> i32 {
        let name = self.name;
        println!("{name} was bought!");
        123
    }
}

/* Strutcs can be taken as the definition of object attributes en OOP
allows to define a new type that acces by name the attribute

*/

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

    ////////////////////////////////////////////////

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
    book.buy();

    #[derive(Debug)]
    struct Person {
        name: String,
        age: Box<u8>,
    }

    let person = Person {
        name: String::from("Alice"),
        age: Box::new(20),
    };

    // `name` is moved out of person, but `age` is referenced
    let Person { name, ref age } = person;

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
