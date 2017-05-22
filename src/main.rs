extern crate gtk;
extern crate gdk;


pub mod gui;

fn main() {
    println!("Starting Game server GUI");
    gui::launch()
}
