use gtk::gdk::ffi::{gdk_key_event_get_keyval, gdk_keyval_name};
use gtk::gdk::KeymapKey;
use gtk::glib::ffi::G_UNICODE_SCRIPT_TIRHUTA;
use gtk::{gdk, prelude::*};
use gtk::{glib, Application, Label, Orientation, Align, Window, AlertDialog, ApplicationWindow, Entry, SearchBar, TextWindowType, EventControllerKey};
use std::collections::HashMap;
use std::process::{Command, Stdio};
use serde_json::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]

// JSON output from hyprctl command.
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

fn parse_json() -> Result<Vec<WorkspaceInfo>> {
    
    let output = Command::new("hyprctl")
        .arg("-j")
        .arg("clients")
        .output()
        .expect("Failed to fetch workspace information.");
    
    let command_out = String::from_utf8_lossy(&output.stdout);
    let struct_json: Vec<WorkspaceInfo> = serde_json::from_str(&command_out)?;
    
    Ok(struct_json) 
}

fn switch_workspaces(workspace_name: String) -> Result<()> {
    let output = Command::new("hyprctl")
        .arg("dispatch")
        .arg("workspace")
        .arg(workspace_name)
        .output()
        .expect("Failed to switch to application/workspace");
    
    Ok(())
}

fn build_ui(app: &Application) {

    let json = parse_json().unwrap();
    
    let gtk_box = gtk::Box::builder()
        .opacity(0.5)
        .orientation(Orientation::Vertical)
        .halign(Align::Center)
        .build();
    
        
    for workspace in json {
        let label = Label::builder()
            .label(format!("{}", workspace.class))
            .build();
        gtk_box.append(&label)
    }
    
    
    let window = gtk::Window::builder()
        .application(app)
        .title("gtk-rs app")
        .child(&gtk_box)
        .build();

    let event_controller = gtk::EventControllerKey::new();
    
    event_controller.connect_key_pressed(|_, key, _, _| {
        match key {
            gdk::Key::Escape => {
                std::process::exit(0);
            }
            _ => { 
                
                let key_val = key.name().unwrap();
                
                if key_val == "a" {
                    println!("{:?}", key.name().unwrap());
                }
            }   
        }
       glib::Propagation::Proceed
    });
    
    window.add_controller(event_controller);
    
    window.set_default_size(400, 400);
    window.present();

}