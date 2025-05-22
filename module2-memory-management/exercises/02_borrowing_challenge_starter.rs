// 1. Processing string data with immutable references
fn get_length(/* TODO: Add parameter for an immutable reference to a String */) -> usize {
    // TODO: Return the length of the string
    0 // Replace this placeholder
}

// 2. Modifying vector data with mutable references
fn add_three_elements(/* TODO: Add parameter for a mutable reference to a Vec<i32> */) {
    // TODO: Add three elements (10, 20, and 30) to the vector
}

// 3. Processing multiple data structures
fn calculate_stats(numbers: &Vec<f64>, strings: &Vec<String>) -> (f64, i32) {
    // TODO: Calculate and return the average of the numbers vector and the count of items in the strings vector
    (0.0, 0) // Replace this placeholder
}

// 4. Borrowing rules demonstration
fn fix_borrowing_issues() {
    let mut data = vec![1, 2, 3];
    
    // TODO: The following code has borrowing issues. Uncomment and fix it.
    // let ref1 = &mut data;
    // let ref2 = &mut data;
    // ref1.push(4);
    // ref2.push(5);
    // println!("Modified data: {:?}", data);
    
    // TODO: Fix another example of borrowing issue
    // let ref3 = &data;
    // let ref4 = &mut data;
    // println!("Data length: {}", ref3.len());
    // ref4.push(6);
}

fn main() {
    // 1. Test immutable reference function
    let test_string = String::from("Hello, Rust borrowing!");
    let length = get_length(/* TODO: Pass the string as an immutable reference */);
    println!("String length: {}", length);
    // Verify the string is still usable after passing as reference
    println!("Original string: {}", test_string);
    
    // 2. Test mutable reference function
    let mut my_vec: Vec<i32> = Vec::new();
    println!("Before function call: {:?}", my_vec);
    add_three_elements(/* TODO: Pass the vector as a mutable reference */);
    println!("After function call: {:?}", my_vec);
    
    // 3. Test multiple references
    let numbers = vec![10.0, 20.0, 30.0, 40.0, 50.0];
    let words = vec![String::from("apple"), String::from("banana"), String::from("cherry")];
    let (average, count) = calculate_stats(/* TODO: Pass appropriate references */);
    println!("Average of numbers: {:.1}, Count of strings: {}", average, count);
    
    // 4. Test the fixed borrowing issues
    fix_borrowing_issues();
}