fn main() {
   println!("Hello, world!");
    println!("{}", plus_one(3));
    println!("{:?}", plus_one(3));
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
