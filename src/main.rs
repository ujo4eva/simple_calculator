use std::io;

enum Operator {

    Addition,
    Subtraction,
    Multiplication,
    Division,
}

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
        
            loop {
            
            println!("Choose an operation:");
            println!("1. Addition");
            println!("2. Subtraction");
            println!("3. Multiplication");
            println!("4. Division");

            let mut choice = String::new();

        io::stdin().read_line(&mut choice).expect("Failed to read line!");
        let choice = choice.trim().parse().unwrap();

                match choice {

                1 => perform_operation(Operator::Addition, num1, num2),
                2 => perform_operation(Operator::Subtraction, num1, num2),
                3 => perform_operation(Operator::Multiplication, num1, num2),
                4 => perform_operation(Operator::Division, num1, num2),
                _ => println!("Invalid operator!"), 

                };

                break;
                
            }
        }
    }

fn perform_operation(operator: Operator, num1: i32, num2: i32) {

    match operator {

        Operator::Addition => println!("The sum is {}", num1 + num2),
        Operator::Subtraction => println!("The difference is {}", num1 - num2),
        Operator::Multiplication => println!("The product is {}", num1 * num2),
        Operator::Division => if num2 == 0 {

            panic!("Division by zero not allowed!");

        } else {

            println!("The quotient is {}", num1 / num2);

        }
    }
}
