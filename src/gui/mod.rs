use gtk::*;
use regex::Regex;


//  GTK+ is not thread-safe. Accordingly, none of this crate's structs implement Send or Sync.

fn configure_window(window: &Window) {
	window.set_title("Connect Four Game Server");
	let(width, height) = (500, 200);
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

fn build_game_window(k: i32, height: i32, width: i32) {
	let game_glade_src = include_str!("game_window.glade");
	let game_builder = Builder::new_from_string(game_glade_src);
	let game_window: Window = game_builder.get_object("game_window").unwrap();
	configure_game_window(&game_window);

	let game_board = Grid::new();
	game_board.set_name("game_grid");
	// game_board.set_column_spacing(35);
	// game_board.set_row_spacing(35);
	game_board.set_row_homogeneous(true);
	game_board.set_column_homogeneous(true);
	for i in 0..width {
		for j in 0..height {
			let image = Image::new_from_file("empty.png");
			let mut name = i.to_string();
			name.push_str(",");
			name.push_str(&j.to_string());
			image.set_name(&name);
			game_board.attach(&image, i, j, 1, 1);
		}
	}

	// add radio buttons
    let mut radio_vec = vec![];
    let base = RadioButton::new_with_label_from_widget(None, "-1");
    for i in 1..width+1 {
    	let btn = RadioButton::new_with_label_from_widget(Some(&base), &i.to_string());
    	btn.set_halign(Align::Center);
    	game_board.attach(&btn, i-1, height+1, 1, 1);
    	radio_vec.push(btn);
    }

	let play_button = Button::new_with_label("Play");
	play_button.set_name("play_btn");

	play_button.connect_clicked(move |_| {
		for button in &radio_vec {
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

	let game_box: Box = game_builder.get_object("game_box").unwrap();
	game_box.pack_start(&game_board, true, true, 20);
	game_box.pack_start(&play_button, false, true, 20);
 
 //    let radio_button_group = vec![col_1, col_2, col_3, col_4, col_5, col_6, col_7];

 	let side_box: Box = game_builder.get_object("side_box").unwrap();
 	let mut k_string = "You need to connect ".to_string();
 	k_string.push_str(&k.to_string());
 	k_string.push_str(" to win!");
 	let k_label = Label::new(Some(k_string.as_str()));
 	side_box.pack_start(&k_label, true, true, 0);

	game_window.show_all();

	// let blue_thing: Image = game_builder.get_object("0,0").unwrap();
	
	// blue_thing.clear();


	// let play_btn: Button = game_builder.get_object("play_btn").unwrap();
	


	let quit_btn: Button = game_builder.get_object("quit_button").unwrap();
	quit_btn.connect_clicked(move |_| {
		main_quit();
    	Inhibit(false);
	});
}

fn connect_to_server(ip_addr: &str) -> Result<(), String> {
	Ok(())
}

pub fn launch() {     

	// first step: initalize GTK
	init().unwrap_or_else(|_| panic!("Panic, unable to initalize GTK!"));	

	// initalize server window

	let server_src = include_str!("server_window.glade");
	let builder = Builder::new_from_string(server_src);
	let server_window: Window = builder.get_object("server_window").unwrap();
	configure_window(&server_window);

	// add closure to connect button to open new (game) screen
	let connect_btn: Button = builder.get_object("connect_btn").unwrap();
	let ip_entry: Entry = builder.get_object("ip_entry").unwrap();
	let warning_label: Label = builder.get_object("warning_label").unwrap();
	let re = Regex::new(r"^\b\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}:\d{1,5}\b$").unwrap();
	connect_btn.connect_clicked(move |_| {
		if let Some(text) = ip_entry.get_text() {
			if re.is_match(&text) {
				// connect_to_server();
				println!("{}", "YAY");
			} else {
				warning_label.set_text("Please enter a valid IP address ie 127.0.0.1:8080");
			}
		} else {
			warning_label.set_text("Nothing in box-o!");
		}
		println!("{}", String::from("Connect button has been clicked"));
	});

	// add closure to quit application when this button is pressed
	let quit_btn: Button = builder.get_object("cancel_btn").unwrap();
	quit_btn.connect_clicked(move |_| {
		main_quit();
    	Inhibit(false);
	});

	server_window.show_all();

	// // initalize main window
	// let glade_src = include_str!("app_window.glade");
	// let builder = Builder::new_from_string(glade_src);
	// let app_window: Window = builder.get_object("window1").unwrap();
	// configure_window(&app_window);

	// // add closure to connect button to open new (game) screen
	// let connect_btn: Button = builder.get_object("button1").unwrap();
	// connect_btn.connect_clicked(move |_| {
	// 	// build and bring game window to view
	// 	build_game_window(8, 4, 4);
	// 	println!("{}", String::from("Connect button has been clicked"));
	// });

	// // bring the window to view and start the application
	// app_window.show_all(); 
	main();
}

