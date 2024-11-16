extern crate gtk4;
extern crate i3ipc;

use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Button};
use i3ipc::I3Connection;

fn main() {
    // Create a new application
    let app = Application::new(Some("com.example.i3commander"), Default::default());

    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

fn build_ui(app: &Application) {
    // Create a new top-level window
    let window = ApplicationWindow::new(app);
    window.set_title(Some("i3 Commander"));
    window.set_default_size(300, 200);

    // Create a button
    let button = Button::with_label("Get i3 Workspaces");
    button.connect_clicked(|_| {
        // Connect to i3 and get workspaces
        let mut connection = I3Connection::connect().unwrap();
        let workspaces = connection.get_workspaces().unwrap();

        // Print workspace names
        for workspace in workspaces.workspaces {
            println!("Workspace: {}", workspace.name);
        }
    });

    // Add button to the window
    window.set_child(Some(&button));

    // Show the window
    window.show();
}
