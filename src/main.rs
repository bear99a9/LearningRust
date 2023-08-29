#![allow(unused_variables)]

mod data_types;
use data_types::data_types;

mod variables;
use variables::variables;

mod operators;
use operators::operators;

mod project;
use project::project;

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
    println!("Control Flow:");
    println!("");

    control_flow();

    println!("---------------------------------------------");

    println!("---------------------------------------------");
    println!("project fn print out:");
    println!("");

    project();


}

