use gtk::{gdk, prelude::*};
use gtk::{glib, Application, Label, Orientation, Align};
use std::collections::HashMap;
use std::process::Command;
use serde_json::Result;
use serde::{Deserialize, Serialize};
use gtk4_layer_shell::{Edge, KeyboardMode, Layer, LayerShell};
use itertools::Itertools;
use std::sync::Mutex;
use lazy_static::lazy_static;

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

const APP_ID: &str = "org.gtk_rs.hyprwindow";

lazy_static::lazy_static! {
    static ref CURRENT_INDEX: Mutex<usize> = Mutex::new(0);
}

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

fn get_duplicate_applications(app_name: char, app_names: &Vec<char>) -> (Vec<String>, HashMap<&char, usize>) {
    let json = parse_json().unwrap();
    
    let counts = app_names.iter().counts();
    let mut workspace_vec = vec![];
    
    if counts[&app_name] > 1 {
        for window in &json {
            for _ in 0..counts[&app_name] {
                if window.class.to_lowercase().chars().next().unwrap() == app_name {
                    let name = serde_json::to_string(window.workspace.get("id").unwrap()).unwrap();
                    workspace_vec.push(name)
                }
            }
        }
    }
    
    let _ = workspace_vec.sort();
    let _ = workspace_vec.dedup();
    
    (workspace_vec, counts)
}

fn get_active_workspace() -> String {
    
    let current_workspace = Command::new("sh")
        .arg("-c")
        .arg("hyprctl activeworkspace -j | jq '.id'")
        .output()
        .expect("Failed to get current workspace id");
    
    let active_workspace_id = String::from_utf8_lossy(&current_workspace.stdout);
    
    active_workspace_id.trim_end().to_owned()
}

fn switch_workspaces(app_name: char, app_names: &Vec<char>) -> Result<()> {
    
    let json = parse_json().unwrap();
    let (workspace_vec, counts) = get_duplicate_applications(app_name, app_names);
    
    let mut workspace_name = "".to_string();
    
    if counts[&app_name] > 1 {
        let mut index = CURRENT_INDEX.lock().unwrap();
        
        if *index >= workspace_vec.len() {
            *index = 0;
        }
        
        let id = &workspace_vec[*index];
        
        if id != get_active_workspace().trim_end() {
            workspace_name = id.to_owned();
        }
        
        *index += 1;
        
    } else {
        for window in json {
            if window.class.to_lowercase().chars().next().unwrap() == app_name {
                workspace_name = serde_json::to_string(window.workspace.get("id").unwrap()).unwrap();
            }
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
        
        
        // get first character in string
        let first_char_app = workspace.class.to_lowercase().chars().next().unwrap();
        let workspace_name = serde_json::to_string(workspace.workspace.get("id").unwrap()).unwrap();
        
        app_names.push(first_char_app);
        
        let label = Label::builder()
            .label(format!("{:} {:}", workspace.class, workspace_name))
            .build();
        
        gtk_box.append(&label)
        
    }
    
    let window = gtk::ApplicationWindow::builder()
        .opacity(0.9)
        .application(app)
        .child(&gtk_box)
        .build();
    
    let event_controller = gtk::EventControllerKey::new();
    
    event_controller.connect_key_pressed(move |_, key, _, _| {
        match key {
            gdk::Key::Super_L => {
                std::process::exit(0);
            }
            _ => {
                
                let mut dedup_app_names = app_names.clone();
                let _ = dedup_app_names.sort();
                let _ = dedup_app_names.dedup();
                
                for app in &dedup_app_names {
                    let key_val = key.name().unwrap().chars().next().unwrap();
                    
                    if key_val == *app {
                        _ = switch_workspaces(key_val, &app_names);
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
    window.set_keyboard_mode(KeyboardMode::Exclusive);
    window.present();

}