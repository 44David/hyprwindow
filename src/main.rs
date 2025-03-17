use gtk::ffi::gtk_window_get_transient_for;
use gtk::gdk::Popup;
use gtk::gio::Menu;
// use gtk::gdk::ffi::{gdk_key_event_get_keyval, gdk_keyval_name};
// use gtk::gdk::KeymapKey;
// use gtk::glib::ffi::G_UNICODE_SCRIPT_TIRHUTA;
// use gtk::glib::GString;
use gtk::{gdk, prelude::*, AlertDialog, Overlay, PopoverMenuBar, TextWindowType};
use gtk::{glib, Application, Label, Orientation, Align, ApplicationWindow, ButtonsType, DialogFlags, MessageType, MessageDialog, Window};
use std::collections::HashMap;
use std::process::Command;
use serde_json::Result;
use serde::{Deserialize, Serialize};
use gtk4_layer_shell::{Edge, Layer, LayerShell};

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

fn switch_workspaces(app_name: char) -> Result<()> {
    
    let json = parse_json().unwrap();
    
    let mut workspace_name = "".to_string();
    for window in json {
        if window.class.to_lowercase().chars().next().unwrap() == app_name {
            workspace_name = serde_json::to_string(window.workspace.get("id").unwrap()).unwrap();
        }
    }
    
    let _output = Command::new("hyprctl")
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
    
    
    let mut app_names = vec![];
    for workspace in &json {
        
        let label = Label::builder()
            .label(format!("{}", workspace.class))
            .build();
        
        // get first character in string
        app_names.push(workspace.class.to_lowercase().chars().next().unwrap());
        gtk_box.append(&label)
    }
    
    let overlay = Overlay::builder()
        .build();
    let window = gtk::ApplicationWindow::builder()
        .opacity(0.9)
        .application(app)
        .child(&gtk_box)
        .build();
    
    
    
    let event_controller = gtk::EventControllerKey::new();
    
    event_controller.connect_key_pressed(move |_, key, _, _| {
        match key {
            gdk::Key::Escape => {
                std::process::exit(0);
            }
            _ => {
                
                for app in &app_names {
                    let key_val = key.name().unwrap().chars().next().unwrap();
                    
                    if key_val == *app {
                        let _ = switch_workspaces(key_val);
                    }
                }
            }  
        }
       glib::Propagation::Proceed
    });
    
    window.add_controller(event_controller);
    window.init_layer_shell();
    window.set_layer(Layer::Overlay);
    window.set_anchor(Edge::Top , true);
    window.grab_focus();
    window.present();

}