use gdk4::Display;
use gtk::prelude::*;
use gtk::{Button, CssProvider};

pub fn create_button(label: &str) -> Button {
    let button = Button::with_label(label);
    button.add_css_class("button_custom");

    let provider = CssProvider::new();
    provider.load_from_string(include_str!("style.css"));

    // Add the provider to the default screen
    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    button
}
