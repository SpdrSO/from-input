use std::{
    str::FromStr,
    process::exit,
};

use from_input::FromInput;

fn main() {
    loop {
        let action = Action::from_input_retry(
            "Choose an action (+ - * / q) >> ",
            |_| eprintln!("[!] Invalid action. Please try again."),
        ).unwrap();

        if action == Action::Quit {
            exit(0);
        }

        let first = f64::from_input_retry(
            "Input first number >> ",
            |_| eprintln!("[!] Invalid number. Please try again."),
        ).unwrap();
        let second = f64::from_input_retry(
            "Input second number >> ",
            |_| eprintln!("[!] Invalid number. Please try again."),
        ).unwrap();

        let result = match action {
            Action::Add => first + second,
            Action::Sub => first - second,
            Action::Mul => first * second,
            Action::Div => first / second,
            Action::Quit => unreachable!(),
        };

        if result.is_finite() {
            println!("Result: {}", result);
        } else {
            eprintln!("[!] Operation error. Please try again.")
        }

        println!();
    }
}

#[derive(PartialEq)]
enum Action {
    Add,
    Sub,
    Mul,
    Div,
    Quit,
}

impl FromStr for Action {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Action::Add),
            "-" => Ok(Action::Sub),
            "*" => Ok(Action::Mul),
            "/" => Ok(Action::Div),
            "q" => Ok(Action::Quit),
            _ => Err(()),
        }
    }
}
