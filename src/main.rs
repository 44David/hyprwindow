use gtk::{prelude::*, ApplicationWindow};
use gtk::{glib, Application, Label, Orientation, Align};
use std::process::Command;

const APP_ID: &str = "org.gtk_rs.window";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    
    app.connect_activate(build_ui);
    
    app.run()
}

fn build_ui(app: &Application) {
    
    let output = Command::new("hyprctl")
        .arg("-j")
        .arg("workspaces")
        .output()
        .expect("Failed to fetch workspaces.");
    
    let command_out = String::from_utf8_lossy(&output.stdout);
    
    
    let label = Label::builder()
        .label(format!("{}", command_out))
            .build();
    
    let gtk_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .spacing(12)
        .halign(Align::Center)
        .build();
    gtk_box.append(&label);
    
    let window = ApplicationWindow::builder()
        .application(app)
        .title("gtk-rs app")
        .child(&gtk_box)
        .build();
    
    window.present();
}