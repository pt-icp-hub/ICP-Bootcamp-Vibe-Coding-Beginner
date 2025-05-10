// Define the Book struct
struct Book {
    // TODO: Add fields for book properties (title, author, year, isbn)
}

// Define a BookStatus enum to track availability
enum BookStatus {
    // TODO: Add variants for different states (Available, Borrowed)
}

// Define a Library struct to manage books
struct Library {
    // TODO: Add fields to store books and their status
}

// TODO: Implement methods for the Library struct
impl Library {
    // Create a new, empty library
    fn new() -> Library {
        // TODO: Implement this function
    }

    // Add a book to the library
    fn add_book(&mut self, book: Book) {
        // TODO: Implement this function
    }

    // Borrow a book from the library
    fn borrow_book(&mut self, isbn: &str) -> Result<&Book, &str> {
        // TODO: Implement this function
    }

    // Return a borrowed book to the library
    fn return_book(&mut self, isbn: &str) -> Result<&Book, &str> {
        // TODO: Implement this function
    }

    // List all books in the library with their status
    fn list_books(&self) {
        // TODO: Implement this function
    }
}

fn main() {
    // Create a new library
    let mut library = Library::new();
    
    // Add several books to the library
    library.add_book(Book::new(
        "The Rust Programming Language",
        "Steve Klabnik and Carol Nichols",
        2018,
        "9781718500440"
    ));
    
    library.add_book(Book::new(
        "Design Patterns",
        "Erich Gamma et al.",
        1994,
        "9780201633610"
    ));
    
    library.add_book(Book::new(
        "Clean Code",
        "Robert C. Martin",
        2008,
        "9780132350884"
    ));
    
    // List all books
    library.list_books();
    
    // Borrow a book
    println!("Borrowing \"Clean Code\"...");
    match library.borrow_book("9780132350884") {
        Ok(_) => println!("Book borrowed successfully!"),
        Err(e) => println!("Error: {}", e),
    }
    println!();
    
    // List all books again to see the updated status
    library.list_books();
    
    // Return the book
    println!("Returning \"Clean Code\"...");
    match library.return_book("9780132350884") {
        Ok(_) => println!("Book returned successfully!"),
        Err(e) => println!("Error: {}", e),
    }
    println!();
    
    // List all books one more time
    library.list_books();
}