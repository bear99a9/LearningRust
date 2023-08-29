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

mod ownership_borrowing;
use ownership_borrowing::ownership_borrowing;

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

    println!("---------------------------------------------");

    println!("---------------------------------------------");
    println!("ownership and borrowing fn print out:");
    println!("");

    ownership_borrowing();

}

