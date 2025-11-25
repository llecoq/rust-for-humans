// A simple enum representing different actions a user can perform
enum Action {
    Login(String),      // contains a username
    Logout,
    SendMessage(String) // contains the message text
}

// fn handle_action(action: Action) {
//     //
// }

fn main() {
    let a1 = Action::Login("Alice".to_string());
    let a2 = Action::SendMessage("Hello, world!".to_string());
    let a3 = Action::Logout;

    // handle_action
}
