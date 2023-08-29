#![allow(unused_variables)]

mod data_types;
use data_types::data_types;

mod variables;
use variables::variables;

mod operators;
use operators::operators;

mod project_part_one;
use project_part_one::project_part_one;

mod control_flow;
use control_flow::control_flow;

fn main() {
    println!("Data types fn print out:");
    println!("");

    data_types();

    println!("---------------------------------------------");
    println!("variables fn print out:");
    println!("");

    variables();

    println!("---------------------------------------------");

    println!("---------------------------------------------");
    println!("operators fn print out:");
    println!("");

    operators();

    println!("---------------------------------------------");

    println!("---------------------------------------------");
    println!("project part 1 fn print out:");
    println!("");

    project_part_one();

    println!("---------------------------------------------");

    println!("---------------------------------------------");
    println!("Control Flow:");
    println!("");

    control_flow();

}

