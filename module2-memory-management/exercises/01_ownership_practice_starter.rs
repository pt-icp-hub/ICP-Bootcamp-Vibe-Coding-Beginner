// Example 1: String ownership
fn example1() {
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{}, world!", s1); // Error: s1 has been moved
}

// Example 2: Function ownership
fn example2() {
    let s = String::from("hello");
    takes_ownership(s);

    println!("After function call: {}", s); // Error: s has been moved
}

fn takes_ownership(some_string: String) {
    println!("Inside function: {}", some_string);
}

// Example 3: Vector ownership
fn example3() {
    let v = vec![1, 2, 3, 4, 5];

    for i in v {
        println!("{}", i);
    }

    // Calculate and print the sum of elements in v
    let sum: i32 = v.iter().sum(); // Error: v has been moved in the for loop
    println!("Sum: {}", sum);
}

fn main() {
    println!("Running Example 1:");
    example1();
    
    println!("\nRunning Example 2:");
    example2();
    
    println!("\nRunning Example 3:");
    example3();
}