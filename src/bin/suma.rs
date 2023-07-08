
fn main(){
println!("{}", suma(15,5));
}


fn suma(x: i32, y: i32) -> i32{
    println!("algo {} " , x);
   // println!("{}", x+y);
     // o sin return la ultima sentencia
    return x+y
}