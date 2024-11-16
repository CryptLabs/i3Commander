extern crate i3ipc;

use i3ipc::I3Connection;

pub fn run_command() {
    let mut connection = I3Connection::connect().unwrap();
    // Run a command
    let command_text = String::from("exec --no-startup-id lutris");

    let outcomes = connection
        .run_command(&command_text)
        .expect("failed to send command")
        .outcomes;

    for outcome in outcomes {
        if outcome.success {
            println!("success");
        } else {
            println!("failure");
            if let Some(e) = outcome.error.as_ref() {
                println!("{}", e);
            }
        }
    }
}
