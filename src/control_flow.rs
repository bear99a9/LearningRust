enum NavigationAids {
    // starts at 1, pero se peude fijar (but it can be set at any point not just the start)
    NDB(u16), 
    VOR(String, f32),
    VORDME(String, f32),
    FIX {name: String, latitude: f32, longitude: f32} // can also be a tuple or any data type
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

    //Match
    let mut animal = "Duck";

    match animal {
        "Duck" => println!("Quack"),
        "Dog" => println!("Bark"),
        _ => println!("All quiet over here"),
    }

    let ndb_freq:u16 = 384;

    match ndb_freq {
        200..=500 =>  println!("NDB frequency is valid"),
        _ =>  println!("NDB frequency is invlaid")
    }

    match ndb_freq{
        ndb_freq if ndb_freq >= 200 && ndb_freq <= 500 => println!("NDB frequency is valid"),
        _ =>  println!("NDB frequency is invlaid")
    }

    // Match with Enum
    let ndb_uwl = NavigationAids::NDB(385);
    let vor_dqn = NavigationAids::VOR(String::from("DQN"), 114.5);
    let vor_dme_sgh = NavigationAids::VORDME(String::from("SGH"), 113.2);
    let fix_tarry = NavigationAids::FIX { name: String::from("Tarry"), latitude: 40.05333, longitude: -83.91367 };

    print_nav_aid(&ndb_uwl);
    print_nav_aid(&vor_dqn);
    print_nav_aid(&vor_dme_sgh);
    print_nav_aid(&fix_tarry);

    // If Let instead of match

    if let animal = "Duck"{ // only take a single = sign
        println!("It is a Duck");
    }

    // Loops 3 loops: For Loop, While Loop, Loop

    let mut counter = 0;
    //just runs for ever if you dont break out
    loop {
        counter += 1;
        if  counter == 3 {
            continue;
        }

        println!("{}", counter);

        if counter == 5 {
            break;
        }
    }

}

fn print_nav_aid(navaid: &NavigationAids){
    match navaid {
        NavigationAids::NDB(khz) => {
            println!("NDB frequency is {} kilohertz", khz);
        }
        NavigationAids::VOR(id,freq ) => {
            println!("VOR indentifier is {} and it's frequency is {} kilohertz", id, freq);
        }
        NavigationAids::VORDME(id,freq ) => {
            println!("VORME indentifier is {} and it's frequency is {} kilohertz", id, freq);
        }
        NavigationAids::FIX { name, latitude, longitude } => {
            println!("FIX {} is at {} latitude and {} longitude", name, latitude, longitude);
        }
    }
}