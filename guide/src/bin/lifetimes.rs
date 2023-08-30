// lifetimes will came to play  when a funtion returns a reference
// or a struct has referenced fields

fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

struct Person<'a> {
    name: &'a str,
    age: u16,
}

fn main() {
    let string1 = String::from("hello");
    let string2 = "world";
    // Creating a block to limit the scope of the borrowed values
    let result = longest(string1.as_str(), string2);

    println!("The longest string is: {}", result);

    // using structs
    let p1 = Person {
        name: "vale",
        age: 50,
    };
    println!("{} and age {}", p1.name, p1.age)
}
