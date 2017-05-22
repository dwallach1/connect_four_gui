use gtk::{self};
use gtk::traits::*;
// use gtk::prelude::*;


//  GTK+ is not thread-safe. Accordingly, none of this crate's structs implement Send or Sync.

fn configure_window(window: &gtk::Window) {
	window.set_title("Connect Four Game Server");
	let(width, height) = (600, 500);
	window.set_default_size(width,height);  
	window.connect_delete_event(|_,_| {
    	gtk::main_quit();
    	gtk::Inhibit(false)
    	
	});
} 

fn configure_game_window(window: &gtk::Window) {
	window.set_title("Connect Four Game Server");
	let(width, height) = (800, 650);
	window.set_default_size(width,height);  
	window.connect_delete_event(|_,_| {
    	gtk::main_quit();
    	gtk::Inhibit(false)
    	
	});
} 



pub fn launch() {     

	// first step: initalize GTK
	gtk::init().unwrap_or_else(|_| panic!("Panic, unable to initalize GTK!"));	

	// initalize main window
	let glade_src = include_str!("app_window.glade");
	let builder = gtk::Builder::new_from_string(glade_src);
	let app_window: gtk::Window = builder.get_object("window1").unwrap();
	configure_window(&app_window);

	// add closure to connect button to open new (game) screen
	let connect_btn: gtk::Button = builder.get_object("button1").unwrap();
	connect_btn.connect_clicked(move |_| {

		// build and bring game window to view
		println!("{}", String::from("Connect button has been clicked"));
		let game_glade_src = include_str!("game_window_2.glade");
		let game_builder = gtk::Builder::new_from_string(game_glade_src);
		let game_window: gtk::Window = game_builder.get_object("window1").unwrap();
		configure_game_window(&game_window);

		// let WIDTH: usize = 6;
		// let HEIGHT: usize = 7;

		// create event spaces
		// let mut events: Vec<gtk::Image> = vec![[WIDTH]; HEIGHT]
		// for i in range(0..WIDTH) {
		// 	for j in range(0..HEIGHT) {
		// 		let image 
		// 		events.append(gtk::Image.)
		// 	}
		// }

		// let img_00: gtk::Image = game_builder.get_object("03").unwrap();
		// img_00.set_from_icon_name("gtk-no", 5);
	
		// let img_00 = gtk::Image.set_from_file("red_piece.png");
		// let board_container: gtk::Fixed = game_builder.get_object("fixed3").unwrap();
		// board_container.add(&img_00);
		// img_00.set_alignment(0.0, 0.0);
  //  		img_00.set_padding(0, 0);

		game_window.show_all();
		// app_window.close();
	    
	});

	// add closure to quit application when this button is pressed
	let quit_btn: gtk::Button = builder.get_object("button2").unwrap();
	quit_btn.connect_clicked(move |_| {
		gtk::main_quit();
    	gtk::Inhibit(false);
	});

	// bring the window to view and start the application
	app_window.show_all(); 
	gtk::main();	
}
