use from_input::{FromInput, FromInputError};

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

impl Coords {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl FromInput for Coords {
    type ParseErr = String;

    fn from_input(prompt: &str) -> Result<Self, FromInputError<Self::ParseErr>> {
        let input = String::from_input(prompt)
            .map_err(|e| match e {
                FromInputError::Io(io) => FromInputError::Io(io),
                FromInputError::Parse(_) => unreachable!(),
            })?;

        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.len() != 2 {
            return Err(from_input::FromInputError::Parse(
                String::from("Need exactly two numbers")
            ));
        }

        let x = parts[0].parse::<i32>()
            .map_err(|e| from_input::FromInputError::Parse(format!("Invalid x: {}", e)))?;
        let y = parts[1].parse::<i32>()
            .map_err(|e| from_input::FromInputError::Parse(format!("Invalid y: {}", e)))?;

        Ok(Self::new(x, y))
    }
}
