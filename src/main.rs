use gtk::{prelude::*, ApplicationWindow};
use gtk::{glib, Application};

const APP_ID: &str = "org.gtk_rs.window";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    
    app.connect_activate(build_ui);
    
    app.run()
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("gtk-rs app")
        .build();
    
    window.present();
}