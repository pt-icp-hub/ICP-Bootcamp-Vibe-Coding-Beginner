// Uncomment each section one at a time and fix the issues

// Problem 1: Fix ownership errors
// fn problem1() {
//     // 1.1: Fix the double-move error
//     let data = vec![1, 2, 3];
//     let x = data;
//     let y = data; // Trying to use data after move
//     println!("{:?} {:?}", x, y);
//
//     // 1.2: Fix the ownership issue with the function
//     let name = String::from("Rust");
//     print_data(name);
//     println!("My name is {}", name); // Trying to use name after move
// }
//
// fn print_data(data: String) {
//     println!("Data: {}", data);
// }

// Problem 2: Fix borrowing conflicts
// fn problem2() {
//     // 2.1: Fix the mutable/immutable borrow conflict
//     let mut numbers = vec![1, 2, 3];
//     let first = &numbers[0];
//     numbers.push(4);
//     println!("First element is: {}", first);
//
//     // 2.2: Fix the multiple mutable borrows
//     let mut data = String::from("Hello");
//     let ref1 = &mut data;
//     let ref2 = &mut data;
//     *ref1 = String::from("Hello, ");
//     *ref2 = ref2.to_string() + "Rust!";
//     println!("Data: {}", data);
// }

// Problem 3: Fix dangling references
// fn problem3() {
//     // 3.1: Fix the dangling reference returned by the function
//     let result = get_string();
//     println!("Result: {}", result);
//
//     // 3.2: Fix the issue with references outliving the data
//     let reference;
//     {
//         let data = vec![1, 2, 3];
//         reference = &data;
//     }
//     println!("Reference: {:?}", reference);
// }
//
// fn get_string() -> &String {
//     let s = String::from("I am a dangling reference");
//     &s
// }

// Problem 4: Fix lifetime problems
// fn problem4() {
//     // 4.1: Fix the function signature to properly handle lifetimes
//     let string1 = String::from("long string is long");
//     let result;
//     {
//         let string2 = String::from("short");
//         result = longest(string1.as_str(), string2.as_str());
//     }
//     println!("Longest string: {}", result);
// }
//
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// Problem 5: Optimize unnecessary cloning
// fn problem5() {
//     // 5.1: Remove unnecessary clones while keeping the code functional
//     let original = String::from("Rust Programming");
//     let cloned = original.clone();
//     let len = calculate_length(cloned.clone());
//     
//     let mut names = Vec::new();
//     names.push(String::from("Alice"));
//     names.push(String::from("Bob"));
//     
//     for name in names.clone() {
//         print_string(name.clone());
//     }
//     
//     println!("Original is still: {}", original);
//     println!("Length was: {}", len);
//     println!("Names: {:?}", names);
// }
//
// fn calculate_length(s: String) -> usize {
//     s.len()
// }
//
// fn print_string(s: String) {
//     println!("{}", s);
// }

fn main() {
    println!("Uncomment and fix each problem section one by one.");
    println!("Once fixed, you can run each problem function from main.");
    
    // Uncomment these as you fix each problem:
    // problem1();
    // problem2();
    // problem3();
    // problem4();
    // problem5();
}