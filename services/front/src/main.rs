use gtk::prelude::*;
use gtk::{Button, Window, WindowType};

fn main() {
    // Initialize GTK application
    gtk::init().expect("Failed to initialize GTK.");

    // Create a new window
    let window = Window::new(WindowType::Toplevel);
    window.set_title("GTK Rust Example");
    window.set_default_size(350, 70);

    // Create a button
    let button = Button::with_label("Click me!");

    // Add button to the window
    window.add(&button);

    // Connect the button clicked event
    button.connect_clicked(|_| {
        println!("Button clicked!");
    });

    // Connect the window closed event
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    // Show all widgets
    window.show_all();

    // Run the GTK main event loop
    gtk::main();
}
