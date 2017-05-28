extern crate gtk;
extern crate gdk;
extern crate regex;


pub mod gui;


fn main() {
    println!("Starting Game server GUI");
    gui::launch()
}
