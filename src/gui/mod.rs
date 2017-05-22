use gtk::{self};
use gtk::traits::*;
use gtk::prelude::*;

//  GTK+ is not thread-safe. Accordingly, none of this crate's structs implement Send or Sync.

fn configure_window(window: &gtk::Window) {
	window.set_title("Connect Four Game Server");
	let(width, height) = (500, 400);
	window.set_default_size(width,height);  
	window.connect_delete_event(|_,_| {
    	gtk::main_quit();
    	gtk::Inhibit(false)
    	
	});
} 

fn configure_game_window(window: &gtk::Window) {
	window.set_title("Connect Four Game Server");
	let(width, height) = (750, 650);
	window.set_default_size(width,height);  
	window.connect_delete_event(|_,_| {
    	gtk::main_quit();
    	gtk::Inhibit(false)
    	
	});
} 

// fn begin_turn() {
	
// }

fn end_turn() {
	let game_glade_src = include_str!("game_window.glade");
	let game_builder = gtk::Builder::new_from_string(game_glade_src);
	let play_btn: gtk::Button = game_builder.get_object("play_btn").unwrap();
	play_btn.hide();
}

fn build_game_window() {
	let game_glade_src = include_str!("game_window.glade");
	let game_builder = gtk::Builder::new_from_string(game_glade_src);
	let game_window: gtk::Window = game_builder.get_object("window1").unwrap();
	configure_game_window(&game_window);

	// add radio buttons
	let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    let hbox_inner = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    hbox_inner.set_spacing(34);

    let base = gtk::RadioButton::new_with_label_from_widget(None, "-1");
    let col_1 = gtk::RadioButton::new_with_label_from_widget(Some(&base), "1");
    let col_2 = gtk::RadioButton::new_with_label_from_widget(Some(&base), "2");
    let col_3 = gtk::RadioButton::new_with_label_from_widget(Some(&base), "3");
    let col_4 = gtk::RadioButton::new_with_label_from_widget(Some(&base), "4");
    let col_5 = gtk::RadioButton::new_with_label_from_widget(Some(&base), "5");
    let col_6 = gtk::RadioButton::new_with_label_from_widget(Some(&base), "6");
    let col_7 = gtk::RadioButton::new_with_label_from_widget(Some(&base), "7");
 
    hbox_inner.pack_start(&col_1, false, false, 0);
    hbox_inner.pack_start(&col_2, false, false, 0);
    hbox_inner.pack_start(&col_3, false, false, 0);
    hbox_inner.pack_start(&col_4, false, false, 0);
    hbox_inner.pack_start(&col_5, false, false, 0);
    hbox_inner.pack_start(&col_6, false, false, 0);
    hbox_inner.pack_start(&col_7, false, false, 0);
 
    let radio_button_group = vec![col_1, col_2, col_3, col_4, col_5, col_6, col_7];
    
    hbox.pack_end(&hbox_inner, false, false, 0);
    let container: gtk::Fixed = game_builder.get_object("fixed1").unwrap();
    container.add(&hbox);
    hbox.set_margin_top(500);
    hbox.set_margin_bottom(50);
    hbox.set_margin_start(50);
    hbox.set_margin_end(150);

	game_window.show_all();

	let play_btn: gtk::Button = game_builder.get_object("play_btn").unwrap();
	play_btn.connect_clicked(move |_| {
		for button in &radio_button_group {
			if button.get_active() {
				// play the move here
				// play_move(int(button.get_label().unwrap());
				// add functionality when we connect oData library and game functionality
				println!("{:?}", button.get_label().unwrap());
				end_turn();
				break;
				// let img_name = "piece_" + str(x) + str(y) ;
				// ler curr_img = game_builder.get_object(img_name).unwrap();
				// if player1 { curr_img.set_image("red_piece.png") }
				// else { curr_img.set_image("blue_piece.png") }	
			}
		}		
		println!("{:?}", String::from("passed out of toggle loop"));
	});


	let quit_btn: gtk::Button = game_builder.get_object("button3").unwrap();
	quit_btn.connect_clicked(move |_| {
		gtk::main_quit();
    	gtk::Inhibit(false);
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
		build_game_window();
		println!("{}", String::from("Connect button has been clicked"));
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

