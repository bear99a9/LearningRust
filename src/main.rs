#![allow(unused_variables)]

fn main() {
    let unused_variable: u32 = 0;

    //Array
    let location_array: [f32;2] = [41.4094069, -81.8546911];

    //Tuple    
    let location_tuple = ("KCLE", 41.4094069, -81.8546911);
    // destructuring a tuple
    let (name, latitude, longitude) = location_tuple;

    println!("Location name: {}, latitude: {}, longitude: {}",
    location_tuple.0, location_tuple.1, location_tuple.2);

    println!("Location name: {}, latitude: {}, longitude: {}",
    name, latitude, longitude);

    //string slice (Immutable)
    let person_name_slice = "Donald Mallard";
    
    // String (Mutable)
    let person_name_string = person_name_slice.to_string();
    let person_name_string = "Donald Mallard".to_string();
    let person_name_string = String::from("Donald Mallard");

    //string slice convert
    let person_name_slice2 = &person_name_string;
    let person_name_slice3 = person_name_string.as_str();

    println!("Hello, world!");
    println!("Hi {}", person_name_slice);
    println!("Hi {} again", person_name_string);

    //Concatentation will always concat into an immutable string
    let duck = "Duck";
    let airline = "Airline";

    let airline_name = [duck, " ", airline].concat();

    let airline_name1 = format!("{} {}", duck, airline);

    let mut slogan = String::new();
    slogan.push_str("We hit the ground"); // both of these add a string to the string
    slogan.push(' ');
    slogan = slogan + "everytime!"; // this adds a string slice not a string to the string

    println!("{}", airline_name);
    println!("{}", airline_name1);
    println!("{}", slogan);
}
