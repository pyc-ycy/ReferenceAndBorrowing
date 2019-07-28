fn main() {
    let reference_to_nothing = dangle();

    /*let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM

    println!("{}, {}, and {}", r1, r2, r3);*/

//    let mut s1 = String::from("hello");
//    change1(&mut s1);
//    println!("s1:{}",s1);
    //change(&s1);
    /*let len = calculate_length(&s1);
    println!("The length  of '{}' is {}.", s1, len);*/
}
fn dangle() -> &String {
    let s = String::from("hello");

    &s
}
/*fn change1(s: &mut String) {
    s.push_str(", I'm trying!!!");
}*/
/*fn change(s:&String) {
    s.push_str(", world");
}*/
/*
fn calculate_length(s: &String) -> usize {
    s.len()
}*/
