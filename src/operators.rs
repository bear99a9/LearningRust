pub fn operators() {
    let squared = i32::pow(8,2);

    // things in parentheses are calculated first
    // division is done before add/ subtract so you get so inside the () divide 12/3 = 4 then 4 + 7 = 11
    // 8+4*2-11+4
    // no exponents of () left but we have * 4*2 = 8
    // 8+8-11+4
    // add/ - are the same so just go down the line
    // answer = 9
    let order_ops = 8+4*2-(12/3+7)+4;

    println!("{}", squared);

    // equality 

    let are_equal_is_true = 1 == 1;
    let are_equal_is_false = 1 == 2;
    let bang_are_not_equal = 1 != 2;

    let is_true = true;
    let is_false = !is_true;
    println!("is_true: {}, is_false: {}", is_true, is_false);

    // AND/ OR

    let have_driver_licence = false;
    let have_passport = true;
    let have_proof = have_driver_licence || have_passport; 

    let have_boarding_pass = true;
    let have_id = have_proof;
    let can_board = have_boarding_pass && have_id;

    println!("Boarding Pass: {}, ID: {}", have_boarding_pass, have_id);
    println!("Can board the plane: {}", can_board);

    // < =< > =>
    let fv = 10;
    let sv = 15;

    let result = 10 > 15; 

    println!("result: {}", result);
}