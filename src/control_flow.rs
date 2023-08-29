enum NavigationAids {
    // starts at 1, pero se peude fijar (but it can be set at any point not just the start)
    NDB = 3, 
    VOR = 2, 
    VORDME = 5,
    //FIX {name: String, latitude: f32, longitude: f32} // can also be a tuple or any data type
}

pub fn control_flow(){

    //IF/ELSE
    let word = "Dog";

    if word == "Duck" {
        println!("Quack");
    }
    else if word == "Dog" {
        println!("Bark");
    }
    else {
        println!("All quiet out here");
    }

    let available_aircraft = "Boeing";
    let min_crew = 7;
    let available_crew = 4;

    if (available_aircraft == "Boeing" || available_aircraft == "Airbus")
        && min_crew <= available_crew {
        println!("OK");
    }

    //Enums
    println!("NDB:\t{}", NavigationAids::NDB as u8);
    println!("VOR:\t{}", NavigationAids::VOR as u8);
    println!("VORDME:\t{}", NavigationAids::VORDME as u8);

    //Option
    let phrase = String::from("Duck Airlines");
    let letter = phrase.chars().nth(15);

    let value: String = match letter {
        Some(character) => character.to_string(),
        None => String::from("No value")
    };

    println!("{}", value);

}
