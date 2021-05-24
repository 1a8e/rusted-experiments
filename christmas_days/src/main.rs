use std::io;

fn main() {
    let mut verse_number = String::new();

    io::stdin()
        .read_line(&mut verse_number)
        .expect("Unable to read verse number");

    let verse_number: usize = verse_number
        .trim()
        .parse()
        .expect("Verse number not an integer");

    let verse_number = verse_number + 1;

    let gifts = [
        "and a partridge in a pear tree",
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

    if verse_number > gifts.len() {
        println!("There are only {} verses :(", gifts.len())
    } else {
        for verse in 1..verse_number {
            generate_verse(verse);
        }
    }
}

fn generate_verse(verse_number: usize) {
    let base = "On the first day of Christmas my true love sent to me";
    println!("{}", base);
    println!("Verse: {}", verse_number);
}
