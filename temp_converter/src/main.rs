use std::io;

fn main() {
    loop {
        println!("Please input a temperature");

        let mut user_input = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        let temp = user_input.trim().remove_last();

        println!("Calculating: {}", temp);

        match user_input.trim().chars().last() {
            Some(num) => match TempUnits::determine_unit(&num) {
                TempUnits::C => match temp.parse::<f64>() {
                    Ok(temp) => {
                        let final_value = ctof(temp);
                        println!("Value converted to Fahrenheit is: {}F", final_value);
                    }
                    Err(_) => {
                        println!("Please input a number");
                        continue;
                    }
                },
                TempUnits::F => match temp.parse::<f64>() {
                    Ok(temp) => {
                        let final_value = ftoc(temp);
                        println!("Value converted to Centigrade is: {}C", final_value)
                    }
                    Err(_) => {
                        println!("Please input a number");
                        continue;
                    }
                },
                TempUnits::None => println!("Did not find a temperature unit"),
            },
            None => println!("Did not find a temperature unit"),
        }
    }
}

// Celsius to Fahrenheit
fn ctof(c: f64) -> f64 {
    (f64::from(c) * (9.0 / 5.0)) + 32.0
}

//Fahrenheit to Celsius
fn ftoc(f: f64) -> f64 {
    (f64::from(f) - 32.0) * (5.0 / 9.0)
}

trait StrExt {
    fn remove_last(&self) -> &str;
}

impl StrExt for str {
    fn remove_last(&self) -> &str {
        match self.char_indices().next_back() {
            Some((i, _)) => &self[..i],
            None => self,
        }
    }
}

enum TempUnits {
    C,
    F,
    None,
}

impl TempUnits {
    fn determine_unit(s: &char) -> TempUnits {
        match s {
            'C' => TempUnits::C,
            'F' => TempUnits::F,
            _ => TempUnits::None,
        }
    }
}
