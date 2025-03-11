use gtk::{prelude::*, ApplicationWindow};
use gtk::{glib, Application, Label, Orientation, Align, Window, AlertDialog};
use std::process::{Command, Stdio};
use serde_json::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct WorkspaceInfo {
    address: String,
    mapped: bool,
    hidden: bool,
    at: Vec<u8>,
    size: Vec<u8>,
    // TO DO Figure out how to put this as type json.
    workspace: Map<u8, id, String, name>,
    floating: bool,
    pseudo: bool,
    monitor: u8,
    class: String,
    title: String,
    initial_class: String,
    initial_title: String,
    pid: u8,
    xwayland: bool,
    pinned: bool,
    fullscreen: u8,
    fullscreen_client: u8,
    grouped: Vec<String>,
    tags: Vec<String>,
    swallowing: String,
    focus_history_id: u8,
    inhibiting_idle: bool
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
    let json: WorkspaceInfo = serde_json::from_str(&command_out)?;
    
    
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