pub fn ownership_borrowing(){
    let original = String::from("original value");
    // original is stored on the heap as it is a string 

    println!("\noriginal: \t\"{}\"\n", original);

    let next = original;
    // when we declare next it points to the data on the heap for original taking ownership of original

    // this causes the error: borrow of moved value: `original`
    println!("{}", original);
}