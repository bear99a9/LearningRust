pub fn ownership_borrowing(){
    let mut original = String::from("original value");
    // original is stored on the heap as it is a string 

    println!("\nOuter original value: \t\"{}\"\n", original);

    //let next = original;
    // when we declare next it points to the data on the heap for original taking over ownership of original
    // original does therefore not exisit any more of the ownership moves
    // rust is fast because all the anayalsis happen at compile time not a run time 

    // this causes the error: borrow of moved value: `original`
    // => error here println!("{}", original);

    // borrowing allows variable to take temp ownership of the data without deallocating the original variable
    
    original = String::from("new value");
    //here we change the data value and it causes no error as it is before the next borrows it

    //{} allow for scope original takes back ownership of the data once next goes out of scope
    {
        let next = &mut original;
        // & lets rust now we are borrowing this by allowing it to read the data from the heap location
        // &mut allows the variable borrowin the data to change the value
        // next = String::from("next value"); this syntax will give you error though due to the pointer
        // to make it work is to dereference the variable 
        *next = String::from("next value"); // the * is basicaly saying go the stored location and chnage the value

    
        //original = String::from("new value1");
        //here we change the data value and it causes an error as rust can't guarentee memeory safety anymore
    
        println!("Inner scope next: \t\"{}\"", next);
        println!("Inner scope original: \t\"{}\"", original);

    }

    println!("Outer original value: \t\"{}\"", original);

}