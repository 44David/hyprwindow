use std::env;
mod minimal_hyprwindow;

fn main() {
    let args: Vec<_> = env::args().collect();
    
    if args[1] == "minimal" {
        minimal_hyprwindow::run();
        
    }
    
}