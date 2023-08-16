fn main() {
    let var1 = 10.2;
    let var2 = 5;

    let result = var1 / var2 as f32;

    println!("{}", result);

    let mut str1 = String::from("some string");
    str1 = priint(str1);

    println!("desde main {}", str1);
}

fn priint(st: String) -> String {
    println!("from inside the func {}", st);
    st
}
