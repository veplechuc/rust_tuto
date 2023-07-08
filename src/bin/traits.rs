pub struct Algo{
    var1: u32,
    var2: u32,
}

pub struct Otra{
    var11: u32,
    var12: u32,


}
pub trait Somthing { // can be taked as a abstrac or interface for structs
    fn una_func(&self);
    
}

impl Somthing for Algo {
    fn una_func(&self) {
        print!("implementation of una_func for Algo struct \n")
    }
}

impl Somthing for Otra {
    fn una_func(&self) {
        print!("implementation of una_func for Otra struct \n")
    }
}

// if we want to goup them we can use a vector

pub struct All {
    pub components: Vec<Box<dyn Somthing>>
}

impl All{
    pub fn print_all(&self){
        for component in self.components.iter() {
            component.una_func();
        }
    }
}



fn main() {
    let all_ptr: All = All{
        components: vec![
            Box::new(Algo { var1:12, var2:15}),
            Box::new(Otra{var11:45, var12:88})
        ]
    };
    all_ptr.print_all();
    
    let d = 30;

    {
        let d = 40;
        println!("inner d is: {d}");
    }

    println!("d is: {d}");

    let mut d = 99;
    
    let e = d;

    d += 1;

    println!("{d}");
    println!("{e}");
    
    let s1 = String::from("Rust"); 
    
    println!("{s1}");
    // s1.push_str("string 1 ");
    
    let mut s2 = s1.clone();
    s2.push_str("string 2");
    
    println!("{s2}");
    println!("{s1}");

}