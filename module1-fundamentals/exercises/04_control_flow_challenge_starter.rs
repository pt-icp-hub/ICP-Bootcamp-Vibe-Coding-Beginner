use std::io;

fn main() {
    // Part 1: FizzBuzz Implementation
    println!("=== FizzBuzz Challenge ===");
    
    // TODO: Implement the FizzBuzz algorithm for numbers 1 to 20
    for i in 1..=20 {
        // TODO: Check if i is divisible by both 3 and 5
        // TODO: Check if i is divisible by 3
        // TODO: Check if i is divisible by 5
        // TODO: Print the number if it's not divisible by 3 or 5
    }
    
    // Part 2: Menu-driven Calculator
    println!("\n=== Calculator ===");
    
    // TODO: Create a variable to control the calculator loop
    let mut running = true;
    
    // TODO: Implement the calculator loop
    while running {
        // TODO: Show the menu options
        println!("Choose an operation:");
        println!("1. Add");
        println!("2. Subtract");
        println!("3. Multiply");
        println!("4. Divide");
        println!("5. Exit");
        
        // TODO: Get the user's choice
        let mut choice = String::new();
        // TODO: Read user input
        
        // TODO: Convert choice to a number (with error handling)
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        };
        
        // TODO: Exit if the user chose option 5
        if choice == 5 {
            // TODO: Set running to false to exit the loop
            break;
        }
        
        // TODO: Get the two input numbers from the user
        // First number
        // TODO: Read first number
        
        // Second number
        // TODO: Read second number
        
        // TODO: Perform the selected operation using match or if statements
        // match choice {
        //    1 => // Handle addition
        //    2 => // Handle subtraction
        //    3 => // Handle multiplication
        //    4 => // Handle division (remember to check for division by zero)
        //    _ => println!("Invalid option. Please try again."),
        // }
        
        // TODO: Ask if the user wants to perform another calculation
        println!("Do you want to perform another calculation? (y/n): ");
        // TODO: Read user's response
        
        // TODO: Set running to false if the user doesn't want to continue
    }
    
    println!("Thank you for using the calculator!");
}