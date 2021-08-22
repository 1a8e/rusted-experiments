extern crate base64;


#[derive(Debug)]
struct StickCircle {
    angle: f32,
    outside: bool,
}

#[derive(Debug)]
struct CodePoint {
    first: StickCircle,
    second: StickCircle,
}

impl CodePoint {
    fn first_as_value(&self) -> u32 {
        if self.first.outside {
            1
        }
        else {
            0
        }
    }
}

impl CodePoint {
    fn second_as_value(&self) -> u32 {
        if self.second.outside {
            match self.second.angle {
                0.5  => 5,
                0.25 => 6,
                1.75 => 7,
                1.5  => 8,
                1.25 => 9,
                _ => panic!()
            }
        }
        else {
            match self.second.angle {
                0.25 => 0,
                1.75 => 1,
                1.5  => 2,
                1.25 => 3,
                0.75 => 4,
                _ => panic!()
            }
        }
    }
}

fn main() {
    let q_codes = [
        // the flag identifier
        // CodePoint {
        //     first: StickCircle { outside: true,  angle: 0.5 },  second: StickCircle { outside: true,  angle: 1.5  } },
        CodePoint {
            first: StickCircle { outside: true,  angle: 0.5 },  second: StickCircle { outside: false, angle: 1.5  } },
        CodePoint {
            first: StickCircle { outside: true,  angle: 0.5 },  second: StickCircle { outside: false, angle: 1.5  } },
        // the hash
        CodePoint {
            first: StickCircle { outside: true,  angle: 0.5 },  second: StickCircle { outside: false, angle: 1.25 } },  // 1
        CodePoint {
            first: StickCircle { outside: false, angle: 1.5 },  second: StickCircle { outside: false, angle: 1.25 } },  // 2
        CodePoint {
            first: StickCircle { outside: false, angle: 1.5 },  second: StickCircle { outside: true,  angle: 0.5  } },  // 3
        CodePoint {
            first: StickCircle { outside: true,  angle: 0.5 },  second: StickCircle { outside: false, angle: 1.25 } },  // 4
        CodePoint {
            first: StickCircle { outside: true,  angle: 0.5 },  second: StickCircle { outside: false, angle: 1.75 } },  // 5
        CodePoint {
            first: StickCircle { outside: true,  angle: 0.5 },  second: StickCircle { outside: false, angle: 1.5  } },  // 6
        CodePoint {
            first: StickCircle { outside: true,  angle: 0.5 },  second: StickCircle { outside: false, angle: 1.5  } },  // 7
        CodePoint {
            first: StickCircle { outside: true,  angle: 0.5 },  second: StickCircle { outside: false, angle: 1.75 } },  // 8
        CodePoint {
            first: StickCircle { outside: false, angle: 1.5 },  second: StickCircle { outside: true,  angle: 0.25 } },  // 9
        CodePoint {
            first: StickCircle { outside: false, angle: 1.5 },  second: StickCircle { outside: false, angle: 1.75 } },  // 10
        CodePoint {
            first: StickCircle { outside: false, angle: 1.5 },  second: StickCircle { outside: false, angle: 1.25 } },  // 11
        CodePoint {
            first: StickCircle { outside: false, angle: 1.5 },  second: StickCircle { outside: true,  angle: 1.25 } },  // 12
        CodePoint {
            first: StickCircle { outside: true,  angle: 0.5 },  second: StickCircle { outside: false, angle: 0.75 } },  // 13
        CodePoint {
            first: StickCircle { outside: false, angle: 1.5 },  second: StickCircle { outside: false, angle: 1.25 } },  // 14
        CodePoint {
            first: StickCircle { outside: false, angle: 1.5 },  second: StickCircle { outside: true,  angle: 0.25 } },  // 15
        CodePoint {
            first: StickCircle { outside: true,  angle: 0.5 },  second: StickCircle { outside: false, angle: 0.25 } },  // 16
        CodePoint {
            first: StickCircle { outside: false, angle: 1.5 },  second: StickCircle { outside: true,  angle: 1.75 } },  // 17
        CodePoint {
            first: StickCircle { outside: true,  angle: 0.5 },  second: StickCircle { outside: false, angle: 0.75 } },  // 18
        CodePoint {
            first: StickCircle { outside: false, angle: 1.5 },  second: StickCircle { outside: true,  angle: 1.25 } },  // 19
        CodePoint {
            first: StickCircle { outside: true,  angle: 0.5 },  second: StickCircle { outside: false, angle: 1.25 } },  // 20
        CodePoint {
            first: StickCircle { outside: false, angle: 1.5 },  second: StickCircle { outside: false, angle: 0.25 } },  // 21
        CodePoint {
            first: StickCircle { outside: true,  angle: 0.5 },  second: StickCircle { outside: false, angle: 1.75 } },  // 22
        CodePoint {
            first: StickCircle { outside: false, angle: 1.5 },  second: StickCircle { outside: false, angle: 1.25 } },  // 23
        CodePoint {
            first: StickCircle { outside: false, angle: 1.5 },  second: StickCircle { outside: true,  angle: 0.5  } },  // 24
        CodePoint {
            first: StickCircle { outside: false, angle: 1.5 },  second: StickCircle { outside: true,  angle: 1.5  } },  // 25
        CodePoint {
            first: StickCircle { outside: true,  angle: 0.5 },  second: StickCircle { outside: false, angle: 0.25 } },  // 26
        CodePoint {
            first: StickCircle { outside: false, angle: 1.5 },  second: StickCircle { outside: true,  angle: 1.5  } },  // 27
        CodePoint {
            first: StickCircle { outside: true,  angle: 0.5 },  second: StickCircle { outside: false, angle: 1.75 } },  // 28
        CodePoint {
            first: StickCircle { outside: true,  angle: 0.5 },  second: StickCircle { outside: false, angle: 1.25 } },  // 29
        CodePoint {
            first: StickCircle { outside: true,  angle: 0.5 },  second: StickCircle { outside: false, angle: 1.25 } },  // 30
        CodePoint {
            first: StickCircle { outside: true,  angle: 0.5 },  second: StickCircle { outside: false, angle: 1.75 } },  // 31
        CodePoint {
            first: StickCircle { outside: false, angle: 1.5 },  second: StickCircle { outside: false, angle: 1.75 } }   // 32
    ];

    for code in q_codes.iter() {
        let digit  = code.first_as_value() * 10 + code.second_as_value();
        let c = char::from_digit(digit, 16);
        print!("{}", c.unwrap());
    }
    println!();
    println!("{:?}", hex_to_bytes(String::from("ccd35dbccb6139e36a7e9d0b358a8bddb1")));
    println!("{:?}", hex_to_base64(String::from("ccd35dbccb6139e36a7e9d0b358a8bddb1")));
}

pub fn hex_to_bytes(hex: String) -> Vec<u8> {
    let mut bytes: Vec<u8> = Vec::new();
    for i in 0..(hex.len() / 2) {
        let res = u8::from_str_radix(&hex[2 * i..2 * i + 2], 16);
        match res {
            Ok(v) => bytes.push(v),
            Err(e) => println!("Unable to decode hex: {}", e),
        };
    };
    bytes
}

pub fn hex_to_base64(hex: String) -> String {
    base64::encode(hex_to_bytes(hex))
}
