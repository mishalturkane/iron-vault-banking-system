use std::io;
use colored::*;

fn main() {
    let master_password = 123;
    let mut chance = 3;

    while chance > 0 {
        let mut user_input = String::new(); // Declare inside the loop to reset on each iteration

        println!("Please enter master_password:");
        io::stdin().read_line(&mut user_input).expect("Failed to read line");

        let user_input: i32 = user_input.trim().parse().expect("Please enter a valid integer");

        if user_input == master_password {
            handle_options();
            break;
        } else {
            chance -= 1;
            if chance == 0 {
                println!("Sorry, you've exhausted all attempts.");
            } else {
                println!("Incorrect password. Please try again. Attempts left: {}", chance.to_string().red());
            }
        }
    }
}

fn handle_options() {
    let mut balance = 0.00;

    loop {
        println!("Your balance is: {}", balance.to_string().red());
        println!("Options:");
        println!("1. Deposit money");
        println!("2. Check balance");
        println!("3. Withdraw money");
        println!("4. Exit");

        let mut option = String::new();
        io::stdin().read_line(&mut option).expect("Failed to read line");

        let option: i32 = match option.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid option. Please enter a number between 1 and 4.");
                continue;
            }
        };

        match option {
            1 => balance += deposit_money(),
            2 => check_balance(balance),
            3 => balance -= withdraw_money(),
            4 => {
                println!("Exiting the program. Goodbye!");
                break;
            }
            _ => println!("Invalid option. Please enter a number between 1 and 4."),
        }
    }
}

fn deposit_money() -> f64 {
    println!("Deposit money option chosen.");
    println!("Please enter the amount to deposit:");

    let mut amount_str = String::new();
    io::stdin().read_line(&mut amount_str).expect("Failed to read line");

    let amount: f64 = match amount_str.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid amount. Please enter a valid number.");
            return 0.0; // Return 0 if parsing fails
        }
    };

    println!("You have deposited: ${}", amount.to_string().green());

    amount // Return the deposited amount
}

fn check_balance(balance: f64) {
    println!("Check balance option chosen.");
    println!("Your balance is: {}", balance.to_string().red());
}

fn withdraw_money() -> f64 {
    println!("Withdraw money option chosen.");
    println!("Please enter the amount to withdraw:");

    let mut amount_str = String::new();
    io::stdin().read_line(&mut amount_str).expect("Failed to read line");

    let amount: f64 = match amount_str.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid amount. Please enter a valid number.");
            return 0.0; // Return 0 if parsing fails
        }
    };

    println!("You have withdrawn: ${}", amount.to_string().green());

    amount // Return the withdrawn amount
}
