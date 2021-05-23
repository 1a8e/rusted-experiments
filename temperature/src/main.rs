use std::io;

fn main() {
    let mut source_temperature = String::new();

    io::stdin()
    	.read_line(&mut source_temperature)
    	.expect("Failed to read source temperature");

    // TODO: add ability to parse C/F and convert both ways

    let source_temperature: f32 = source_temperature
    	.trim()
    	.parse()
    	.expect("Index entered was not a mumber");

    let destination_temperature = (source_temperature - 32.0) * (5.0/9.0);

    println!("{}", destination_temperature);
}
