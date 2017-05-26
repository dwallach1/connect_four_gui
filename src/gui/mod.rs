// use gtk::{self};
// use gtk::traits::*;
// use gtk::prelude::*;
use gtk::*;


//  GTK+ is not thread-safe. Accordingly, none of this crate's structs implement Send or Sync.

fn configure_window(window: &Window) {
	window.set_title("Connect Four Game Server");
	let(width, height) = (500, 400);
	window.set_default_size(width,height);  
	window.connect_delete_event(|_,_| {
    	main_quit();
    	Inhibit(false)
    	
	});
} 

fn configure_game_window(window: &Window) {
	window.set_title("Connect Four Game Server");
	let(width, height) = (750, 650);
	window.set_default_size(width,height);  
	window.connect_delete_event(|_,_| {
    	main_quit();
    	Inhibit(false)
    	
	});
} 

// fn begin_turn() {
	
// }

fn end_turn() {
	let game_glade_src = include_str!("game_window.glade");
	let game_builder = Builder::new_from_string(game_glade_src);
	let play_btn: Button = game_builder.get_object("play_btn").unwrap();
	play_btn.hide();
}

fn build_game_window() {
	let game_glade_src = include_str!("test.glade");
	let game_builder = Builder::new_from_string(game_glade_src);
	let game_window: Window = game_builder.get_object("game_window").unwrap();
	configure_game_window(&game_window);

	let k = 6;
	let game_board = Grid::new();
	game_board.set_column_spacing(35);
	game_board.set_row_spacing(35);

	for i in 0..k {
		for j in 0..k {
			let image = Image::new_from_file("empty.png");
			game_board.attach(&image, i, j, 1, 1);
		}
	}

	let play_button = Button::new_with_label("Play");

	// add radio buttons
    let radio_box = Box::new(Orientation::Horizontal, 0);
    let base = RadioButton::new_with_label_from_widget(None, "-1");
    for i in 1..k+1 {
    	let btn = RadioButton::new_with_label_from_widget(Some(&base), &i.to_string());
    	radio_box.pack_start(&btn, false, false, 25);
    }

	let game_box: Box = game_builder.get_object("game_box").unwrap();
	game_box.pack_start(&game_board, true, true, 20);
	game_box.pack_start(&radio_box, true, true, 20);
	game_box.pack_start(&play_button, false, true, 20);
 
 //    let radio_button_group = vec![col_1, col_2, col_3, col_4, col_5, col_6, col_7];
    
 //    hbox.pack_end(&hbox_inner, false, false, 0);
 //    let container: Fixed = game_builder.get_object("fixed1").unwrap();
 //    container.add(&hbox);
 //    hbox.set_margin_top(500);
 //    hbox.set_margin_bottom(50);
 //    hbox.set_margin_start(50);
 //    hbox.set_margin_end(150);

	game_window.show_all();

	// let play_btn: Button = game_builder.get_object("play_btn").unwrap();
	// play_btn.connect_clicked(move |_| {
	// 	for button in &radio_button_group {
	// 		if button.get_active() {
	// 			// play the move here
	// 			// play_move(int(button.get_label().unwrap());
	// 			// add functionality when we connect oData library and game functionality
	// 			println!("{:?}", button.get_label().unwrap());
	// 			end_turn();
	// 			break;
	// 			// let img_name = "piece_" + str(x) + str(y) ;
	// 			// ler curr_img = game_builder.get_object(img_name).unwrap();
	// 			// if player1 { curr_img.set_image("red_piece.png") }
	// 			// else { curr_img.set_image("blue_piece.png") }	
	// 		}
	// 	}		
	// 	println!("{:?}", String::from("passed out of toggle loop"));
	// });


	let quit_btn: Button = game_builder.get_object("quit_button").unwrap();
	quit_btn.connect_clicked(move |_| {
		main_quit();
    	Inhibit(false);
	});

}


pub fn launch() {     

	// first step: initalize GTK
	init().unwrap_or_else(|_| panic!("Panic, unable to initalize GTK!"));	

	// initalize main window
	let glade_src = include_str!("app_window.glade");
	let builder = Builder::new_from_string(glade_src);
	let app_window: Window = builder.get_object("window1").unwrap();
	configure_window(&app_window);

	// add closure to connect button to open new (game) screen
	let connect_btn: Button = builder.get_object("button1").unwrap();
	connect_btn.connect_clicked(move |_| {
		// build and bring game window to view
		build_game_window();
		println!("{}", String::from("Connect button has been clicked"));
	});

	// add closure to quit application when this button is pressed
	let quit_btn: Button = builder.get_object("button2").unwrap();
	quit_btn.connect_clicked(move |_| {
		main_quit();
    	Inhibit(false);
	});

	// bring the window to view and start the application
	app_window.show_all(); 
	main();	
}

