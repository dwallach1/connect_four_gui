extern crate gtk;
extern crate gdk;
extern crate regex;
#[macro_use] extern crate serde_json;  
extern crate hyper;                    
                 


pub mod gui;


fn main() {
    println!("Starting Game server GUI");
    gui::launch()
}
