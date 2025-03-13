use gtk::{prelude::*, ApplicationWindow};
use gtk::{glib, Application, Label, Orientation, Align, Window, AlertDialog};
use std::collections::HashMap;
use std::process::{Command, Stdio};
use serde_json::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct WorkspaceInfo {
    address: String,
    mapped: bool,
    hidden: bool,
    at: Vec<u32>,
    size: Vec<u32>,
    workspace: HashMap<String, serde_json::Value>,
    floating: bool,
    pseudo: bool,
    monitor: u8,
    class: String,
    title: String,
    initialClass: String,
    initialTitle: String,
    pid: u32,
    xwayland: bool,
    pinned: bool,
    fullscreen: u32,
    fullscreenClient: u32,
    grouped: Vec<String>,
    tags: Vec<String>,
    swallowing: String,
    focusHistoryID: u32,
    inhibitingIdle: bool
}

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
    let json: Vec<WorkspaceInfo> = serde_json::from_str(&command_out)?;
    for workspace in json {
        println!("{}", workspace.title);
        
    }
    
    
    Ok(())
}

fn build_ui(app: &Application) {
    
    // Contains text
    // let label = Label::builder()
    //     .label(format!("{}", command_out))
    //     .build();
        
    if let Err(e) = parse_json() {
        eprintln!("Error parsing JSON: {}", e)
    }

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