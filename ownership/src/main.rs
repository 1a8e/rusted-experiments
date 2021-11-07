fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);
    takes_ownership(s2);

    let x = 5;
    makes_copy(x);
}


fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}


fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}
