extern crate gio;
extern crate gtk;

use std::env;

use gio::{ApplicationExt, ApplicationExtManual, ApplicationFlags};
use gtk::{
    Application,
    ApplicationWindow,
    WidgetExt,
    GtkWindowExt,
};

fn main() {
    let application = Application::new("com.github.rust-by-     example", ApplicationFlags::empty())
        .expect("Application initialization failed");
    application.connect_startup(|application| {
        build_ui(application);
    });
    application.run(&env::args().collect::<Vec<_>>());
}