use std::io;

fn main() {
    let mut iterations = String::new();

    io::stdin()
        .read_line(&mut iterations)
        .expect("unable to read iterations");

    let iterations: usize = iterations
        .trim()
        .parse()
        .expect("iterations not an integer");

    let mut inputs = String::new();

    io::stdin()
        .read_line(&mut inputs)
        .expect("unable to read inputs");

    let mut numbers: Vec<u8> = Vec::new();

    for input in inputs.split(' ') {
        numbers.push(
            input
                .trim()
                .parse::<u8>()
                .expect("unable to parse inputs to numbers")
        )
    }

    solver(iterations, numbers);
}

fn solver(iterations: usize, numbers: Vec<u8>) {
    println!("{}", iterations);
    println!("{:?}", mumbers);
}
