use std::io;
use ordinal::Ordinal;

const GIFTS: [&str; 12] = [
        "a partridge in a pear tree",
        "two turtle doves",
        "three French hens",
        "four calling birds",
        "five gold rings",
        "six geese a-laying",
        "seven swans a-swimming",
        "eight maids a-milking",
        "nine ladies dancing",
        "ten lords a-leaping",
        "eleven pipers piping",
        "twelve drummers drumming",
    ];



fn main() {
    let mut verse_limit = String::new();

    io::stdin()
        .read_line(&mut verse_limit)
        .expect("Unable to read verse number");

    let verse_limit: usize = verse_limit
        .trim()
        .parse()
        .expect("Verse number not an integer");

    if verse_limit > GIFTS.len() {
        println!("There are only {} verses :(", GIFTS.len())
    } else {
        let verse_limit = verse_limit + 1;
        for verse_number in 1..verse_limit {
            generate_verse(verse_number, GIFTS);
        }
    }
}

fn generate_verse(verse_number: usize, gifts_var: [&str; 12]) {
    println!("On the {} day of Christmas my true love sent to me", Ordinal(verse_number).to_string());

    for verse in (1..verse_number).rev() {
        println!("{}", gifts_var[verse]);
    }

    if verse_number > 1 {
        println!("and {}", gifts_var[0]);
    } else {
        println!("{}", gifts_var[0]);
    }
}
