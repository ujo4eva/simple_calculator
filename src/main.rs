use std::io;

fn main() {

    loop {
        
        println!("Enter the first number(or \"exit\" to quit):");
        let mut num1 = String::new();

        io::stdin()
            .read_line(&mut num1)
            .expect("Failed to read line!");

            if num1.to_lowercase() == "exit" {

                panic!("Bye :)");
    
            }
        
        let num1: i32 = num1
            .trim()
            .parse()
            .expect("Invalid value for num1!");

        println!("Enter the second number(or \"exit\" to quit):");
        let mut num2 = String::new();

        io::stdin()
            .read_line(&mut num2)
            .expect("Failed to read line!");

            if num2.to_lowercase() == "exit" {

                panic!("Bye :)");
    
            }
        
        let num2: i32 = num2
            .trim()
            .parse()
            .expect("Invalid value for num2!");

        println!("Enter the operator(or \"exit\" to quit):");
        let mut operator = String::new();

        io::stdin()
            .read_line(&mut operator)
            .expect("Failed to read line!");

            if operator.to_lowercase() == "exit" {

                panic!("Bye :)");
    
            }

        let operator: char = operator
            .chars()
            .next()
            .expect("Invalid operator!");

        match operator {

            '+' => {println!("The sum of {} and {} is {}.", num1, num2, num1 + num2)},
            '-' => {println!("The difference between {} and {} is {}.", num1, num2, num1 - num2)},
            '*' => {println!("The product of {} and {} is {}.", num1, num2, num1 * num2)},
            '/' => {

                if num2 == 0 {

                    println!("They used to divide by zero in your village abi?");

                } else {
                    
                    println!("The quotient of {} and {} is {}.", num1, num2, num1 / num2);

                }
            },

            _ => {println!("Invalid calculation!");}
        }
    }
}