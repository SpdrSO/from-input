use std::str::FromStr;

use from_input::FromInput;

fn main() {
    let coords = Coords::from_input("Enter the coordinates >> ").unwrap();
    println!("{:?}", coords);
}

#[derive(Debug)]
#[allow(dead_code)]
struct Coords {
    x: i32,
    y: i32,
}

impl FromStr for Coords {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();

        if parts.len() != 2 {
            return Err(String::from("Need exactly two numbers"));
        }

        let x = parts[0].parse().map_err(|e| format!("Invalid x: {}", e))?;
        let y = parts[1].parse().map_err(|e| format!("Invalid y: {}", e))?;

        Ok(Coords { x, y })
    }
}
