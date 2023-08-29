#![allow(dead_code)]
// rust dont have on the std library la logging "package"

// need to use log from the crates.io

use log::{debug, info};

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
    grade: String,
}
impl Default for Person {
    fn default() -> Person {
        debug!("called Default impl for Person");
        Person {
            name: "Alfred".to_string(),
            age: 0,
            grade: "Junior".to_string(),
        }
    }
}
fn main() {
    // we need to use the ! in order to call the specific macro
    //use the default imple to "initialize" the instance
    env_logger::init(); // this initialize the logger an where to put the nfo
    let tmp_all_default = Person {
        ..Person::default()
    };
    debug!("Person created");
    let tmp_with_name = Person {
        name: "Sam".to_string(),
        grade: "Master".to_string(),
        ..Person::default() // dont put "," here it will trow an error
    };

    println!("tmp_all_defaul {:#?}", tmp_all_default);
    println!("tmp_with_name {:?}", tmp_with_name);
}
