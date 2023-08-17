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
}