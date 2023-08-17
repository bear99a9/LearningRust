pub fn variables(){
    //super strongly typed
    let my_variable_name: u32 = 0;

    //inferred
    let my_inferred_variable = 0; 

    // unused variable fix it with an underscore
    let _unused_variable = "I dont get used";

    // casting variables

    let float_thirty_two: f32 = 17.2;
    let unsigned_eight: u8 = 5;

    // error as we are not explicitly casting
    // let result = float_thirty_two / unsigned_eight;

    // no error as casting properly
    let result = float_thirty_two / unsigned_eight as f32;
    println!("{}", result);

    // cast u8 to char
    let number: u8 = 65;
    let letter: char = number as char;

    println!("{}", letter);

    // Mutability
    
    //mut lets you marks the variable as a mutable
    let mut changeable_variable = 500;
    println!("{}", changeable_variable);
    changeable_variable = 400;
    println!("{}", changeable_variable);

    // Variable Lifetime

    //scope allows you to set the lifetime of the variable 

    let scope_test = "Outer scope";

    println!("{}", scope_test);
    {
        // the variable inside the code block only exists here and not outside 
        let scope_test = "Inner scope";
        println!("{}", scope_test);
    }

    println!("{}", scope_test);


}