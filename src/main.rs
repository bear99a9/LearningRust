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

    println!("Hello, world!");
}
