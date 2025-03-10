use gtk::{prelude::*, ApplicationWindow};
use gtk::{glib, Application, Label, Orientation, Align, Window, AlertDialog};
use std::process::{Command, Stdio};
use std::str::FromStr;
use serde_json::Result;
use serde::{Deserialize, Serialize};


const APP_ID: &str = "org.gtk_rs.window";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    
    app.connect_activate(build_ui);
    
    app.run()
}

fn parse_json() -> Result<()> {
    
    let output = Command::new("hyprctl")
        .arg("-j")
        .arg("clients")
        .output()
        .expect("Failed to fetch workspace information.");
    
        
    let command_out = String::from_utf8_lossy(&output.stdout);
    let json: serde_json::Value = serde_json::Value::from_str(&command_out).unwrap();
    
    println!("{}", json);
    
    Ok(())
}

fn build_ui(app: &Application) {
    
    // Contains text
    // let label = Label::builder()
    //     .label(format!("{}", command_out))
    //     .build();
        
    _ = parse_json();
    
    let gtk_box = gtk::Box::builder()
        .opacity(0.5)
        .orientation(Orientation::Vertical)
        .halign(Align::Center)
        .build();
    // gtk_box.append(&label);
        
    let window = gtk::Window::builder()
        .application(app)
        .title("gtk-rs app")
        .child(&gtk_box)
        .build();
    
        
    window.present();
    
}