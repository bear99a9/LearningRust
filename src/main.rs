#![allow(unused_variables)]

mod data_types;
use data_types::data_types;

mod variables;
use variables::variables;

fn main() {
    println!("Data types fn print out:");
    println!("");

    data_types();

    println!("---------------------------------------------");
    println!("variables fn print out:");
    println!("");

    variables();

    println!("---------------------------------------------");

}

