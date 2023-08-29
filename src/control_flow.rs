enum NavigationAids {
    NDB, // starts at 1
    VOR, // 2
    VORDME // 3
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

}
