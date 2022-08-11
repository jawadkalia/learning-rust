#![allow(unused)]

use std::{cmp::Ordering, io, io::Error, io::ErrorKind};

use rand::Rng;

pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Result<Guess, Error> {
        match (1..=100).contains(&value) {
            true => Ok(Guess { value }),
            false => Err(Error::new(
                ErrorKind::InvalidInput,
                "put it in between 1 to 100",
            )),
        }
    }

    pub fn value(&self) -> u32 {
        self.value
    }
}

fn main() {
    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        match guess.trim().parse() {
            Ok(value) => match Guess::new(value) {
                Ok(number) => match number.value().cmp(&secret_number) {
                    Ordering::Less => println!("number should be bigger"),
                    Ordering::Equal => {
                        println!("You guessed it");
                        break;
                    }
                    Ordering::Greater => println!("Number should be smaller"),
                },
                Err(e) => {
                    println!("Guess not in range");
                    continue;
                }
            },
            Err(_) => {
                println!("Please input a number");
                continue;
            }
        };
    }
}
