extern crate gtk4;
extern crate i3ipc;

use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Button};
use i3ipc::I3Connection;

mod command_loop;
mod run_command;

fn main() {
    // Create a new application
    let app = Application::new(Some("com.example.i3commander"), Default::default());

    app.connect_activate(build_ui);

    // Run the application
    app.run();

    //command_loop::run();
}

fn build_ui(app: &Application) {
    // Create a new top-level window
    let window = ApplicationWindow::new(app);
    window.set_title(Some("i3 Commander"));
    window.set_default_size(300, 200);

    // Create a button
    let button = Button::with_label("Click here");
    button.connect_clicked(|_| {
        // Connect to i3 and get workspaces
        let mut connection = I3Connection::connect().unwrap();
        // Get the version
        let i3version = connection.get_version();
        println!("Workspace: {}", i3version.unwrap().human_readable);
        // Run a command
        run_command::run_command();
    });

    // Add button to the window
    window.set_child(Some(&button));

    // Show the window
    window.show();
}
