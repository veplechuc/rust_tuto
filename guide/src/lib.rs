#![allow(dead_code)]
// must refer to a file
pub mod file_mod1;
// this case refers to a folder an inside a mod.rs
pub mod maths;

pub mod my_mod;

#[path = "bin/another_mod/math2.rs"]
pub mod math2;

pub mod structs_def;
