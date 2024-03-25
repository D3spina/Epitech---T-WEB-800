use gdk4::Display;
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Button, CssProvider};
mod gui;

const APP_ID: &str = "org.gtk_rs.Css1";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to signals
    app.connect_startup(|_| load_css());
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn load_css() {
    // Load the CSS file and add it to the provider
    let provider = CssProvider::new();
    provider.load_from_string(include_str!("./gui/style.css"));

    // Add the provider to the default screen
    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn build_ui(app: &Application) {
    // Create button
    let button_custom = gtk::Button::new();
    //.margin_top(12)
    //.margin_bottom(120)
    //.margin_start(12)
    //.margin_end(12)
    //.build();

    //button_custom.set_size_request(20, 30);
    //button_custom.add_css_class("button_custom");
    //button_custom.can_shrink();

    let boxContainer = gtk::Box::new(gtk::Orientation::Vertical, 20);
    boxContainer.set_size_request(500, 100);
    boxContainer.add_css_class("box_container");

    // boxContainer.append(&button_custom);

    // Create a new window and present it
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(&button_custom)
        .default_width(800)
        .default_height(800)
        .build();
    window.present();
}
