use std::{
    str::FromStr,
    process::exit,
};

use from_input::{FromInput, FromInputError};

fn main() {
    loop {
        let action = prompt_until_valid::<Action>("Choose an action (+ - * / q) >> ");
        if action == Action::Quit {
            exit(0);
        }

        let first = prompt_until_valid::<f64>("Input first number >> ");
        let second = prompt_until_valid::<f64>("Input second number >> ");

        println!(
            "Result: {}",
            match action {
                Action::Add => first + second,
                Action::Sub => first - second,
                Action::Mul => first * second,
                Action::Div => first / second,
                Action::Quit => unreachable!(),
            },
        );

        println!();
    }
}

fn prompt_until_valid<T: FromInput>(prompt: &str) -> T {
    return loop {
        match T::from_input(prompt) {
            Ok(v) => break v,
            Err(FromInputError::Parse(_)) => println!("[Invalid input. Please, try again.]"),
            Err(FromInputError::Io(e)) => panic!("[{}]", e),
        }
    };
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
