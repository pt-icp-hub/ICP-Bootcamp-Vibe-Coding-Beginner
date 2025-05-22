// Starter code for the Rust Task Manager challenge

// Task status enum
enum TaskStatus {
    Pending,
    Completed,
}

// Task struct to store task information
struct Task {
    id: u32,
    title: String,
    description: String,
    due_date: Option<String>, // Consider using a proper date type in your implementation
    status: TaskStatus,
}

// TaskManager to handle operations on tasks
struct TaskManager {
    tasks: Vec<Task>,
    next_id: u32,
}

impl TaskManager {
    // Create a new TaskManager
    fn new() -> TaskManager {
        TaskManager {
            tasks: Vec::new(),
            next_id: 1,
        }
    }
    
    // Add a new task to the task manager
    fn add_task(&mut self, title: String, description: String, due_date: Option<String>) -> &Task {
        let task = Task {
            id: self.next_id,
            title,
            description,
            due_date,
            status: TaskStatus::Pending,
        };
        
        self.next_id += 1;
        self.tasks.push(task);
        self.tasks.last().unwrap()
    }
    
    // TODO: Implement methods for listing, completing, deleting tasks
    // TODO: Implement methods for saving to and loading from a file
    // TODO: Implement methods for filtering tasks
}

// Command enum to represent user commands
enum Command {
    Add { title: String, description: String, due_date: Option<String> },
    List,
    Complete { id: u32 },
    Delete { id: u32 },
    Save { filename: String },
    Load { filename: String },
    Quit,
    Unknown,
}

fn main() {
    // Initialize task manager
    let mut task_manager = TaskManager::new();
    
    // Main application loop
    loop {
        // TODO: Get user command
        
        // TODO: Process command
        
        // TODO: Exit if command is Quit
        break; // Temporary break to avoid infinite loop
    }
}