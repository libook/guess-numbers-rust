use rand;
use regex::Regex;
use std::io;
use rand::Rng;

fn main() {
    const NUMBER_OF_NUMBERS: usize = 4;
    let number_regex = Regex::new(r"^\d{4}$").unwrap();

    /*
    Generate 4 numbers.
    */
    let mut secret_numbers: [u8; 4] = [0, 0, 0, 0];
    let mut current_index: usize = 0;
    while current_index < secret_numbers.len() {
        secret_numbers[current_index] = rand::thread_rng().gen_range(0, 10) as u8;// 0-9

        let mut compare_index: usize = 0;
        let mut passed: bool = true;
        while compare_index < current_index {
            if secret_numbers[current_index] == secret_numbers[compare_index] {
                passed = false;
                break;
            } else {
                compare_index += 1;
            }
        }

        if passed {
            current_index += 1;
        }
    }

    let mut result_history = String::new();

    println!("Please input 4 non-repeating numbers.");

    loop {
        let mut guess_number_string = String::new();

        io::stdin().read_line(&mut guess_number_string).expect("Can not read your input.");

        println!();

        guess_number_string.pop();// Remove '\n' in the end.

        /*
        Check the length.
        */
        if guess_number_string.len() != NUMBER_OF_NUMBERS {
            println!("Please input 4 non-repeating numbers.");
            continue;
        }

        /*
        Check if characters are all number.
        */
        if !number_regex.is_match(&guess_number_string) {
            println!("Please input 4 non-repeating numbers.");
            continue;
        }

        /*
        Convert to number array.
        */
        let mut numbers: [u8; 4] = [0, 0, 0, 0];
        let guess_number_character_vector: Vec<char> = guess_number_string.chars().collect();
        let mut index: usize = 0;
        while index < numbers.len() {
            numbers[index] = match guess_number_character_vector[index].to_digit(10) {
                Some(number) => number as u8,
                None => {
                    println!("Can not convert character to number.");
                    std::process::exit(1);
                }
            };
            index += 1;
        }


        /*
        Check if non-repeating.
        */
        let mut current_index: usize = 0;
        let mut failed: bool = false;
        while current_index < numbers.len() {
            let mut compare_index: usize = current_index + 1;
            while compare_index < numbers.len() {
                if numbers[current_index] == numbers[compare_index] {
                    failed = true;
                    break;
                }
                compare_index += 1;
            }
            if failed {
                break;
            } else {
                current_index += 1;
            }
        }
        if failed {
            println!("Please input 4 non-repeating numbers.");
            continue;
        }

        /*
        Compare and count.
        */
        let mut right_position: usize = 0;
        let mut wrong_position: usize = 0;
        let mut number_index: usize = 0;
        while number_index < numbers.len() {
            let mut secret_number_index: usize = 0;
            while secret_number_index < secret_numbers.len() {
                if numbers[number_index] == secret_numbers[secret_number_index] {
                    if number_index == secret_number_index {
                        right_position += 1;
                    } else {
                        wrong_position += 1;
                    }
                    break;
                } else {
                    secret_number_index += 1;
                }
            }
            number_index += 1;
        }

        /*
        Output result.
        */
        for number in numbers.iter() {
            result_history.push_str(&number.to_string());
        }
        result_history.push_str("\tRight position: ");
        result_history.push_str(&right_position.to_string());
        result_history.push_str("\tWrong position: ");
        result_history.push_str(&wrong_position.to_string());
        result_history.push_str("\n");

        print!("{}", result_history);

        if right_position == NUMBER_OF_NUMBERS {
            println!("You win!");
            break;
        }
    }
}
