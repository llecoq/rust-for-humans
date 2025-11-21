// A simple enum representing different actions a user can perform
enum Action {
    Login(String),      // contains a username
    Logout,
    SendMessage(String) // contains the message text
}

fn handle_action(action: Action) {
    match action {
        Action::Login(name) => {
            // This arm extracts the data stored in the enum
            println!("User '{}' logged in.", name);
        }

        Action::Logout => {
            println!("User logged out.");
        }

        Action::SendMessage(text) => {
            println!("Message sent: {}", text);
        }
    }
}

fn main() {
    let a1 = Action::Login("Alice".to_string());
    let a2 = Action::SendMessage("Hello, world!".to_string());
    let a3 = Action::Logout;

    handle_action(a1);
    handle_action(a2);
    handle_action(a3);
}
