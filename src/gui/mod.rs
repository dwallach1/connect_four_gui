use gtk::*;
use hyper::Client;
use hyper::client::Response;                     
use hyper::status::StatusCode;         
use serde_json::{Value, from_reader};  
use regex::Regex;
use std::io::Read;
use std::str::FromStr;

// is each game a thread ?? if so this will work
use std::time::Duration;
use std::thread::sleep;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Player {
	One,
	Two,
}

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

fn poll_server(curr_board: &str, game_id: i32, ip_addr: &str) {
	let client = Client::new(); 
	let url = &format!("http://{}/api/connect_four.svc/Games({})", ip_addr, game_id);	
	loop {
		let response = client.get(url).send().unwrap();
		let data: Value = from_reader(response).expect("Unable to parse response!");
		let server_board = data["board"].as_str().expect("Unable to parse id!");  
		if curr_board == server_board {
			sleep(Duration::new(20, 0));
		} else { break; }
	}
}



fn play_move(col: usize, id: usize, ip_addr: &str) {
	// tell server to create new game
	let client = Client::new();   

	// get before board                              
	let get_url = &format!("http://{}/api/connect_four.svc/Games({})", ip_addr, id);
	let res = client.get(get_url).send().unwrap();                                      
	assert_eq!(res.status, StatusCode::Ok);                                                                                                                     
	let data: Value = from_reader(res).expect("Unable to parse response!");         
	let prior_board = data["board"].as_str().expect("Unable to parse id!");  

	// send move 
	let post_url = &format!("http://{}/api/connect_four.svc/play_move", ip_addr);
	let value = json!({                                                             
	    "id": id,                                                                   
	    "move": col                                                                  
	});                                                                             
	client.post(post_url).body(&value.to_string()).send().unwrap();
	println!("playing move col: {}, id: {}", col, id);
	  
	// get post board                              
	let res = client.get(get_url).send().unwrap();                                      
	assert_eq!(res.status, StatusCode::Ok);                                         
	let data: Value = from_reader(res).expect("Unable to parse response!");         
	let post_board = data["board"].as_str().expect("Unable to parse id!"); 

	assert_eq!(false, post_board == prior_board);
}

fn get_game(game_id: &str, ip_addr: &str) -> Result<Response, &'static str> {
	let client = Client::new();

	let url = &format!("http://{}/api/connect_four.svc/Games({})", ip_addr, game_id); 
	println!("{}", url);                                                            
	let res_option = client.get(url).send();
	match res_option {
		Ok(response) => Ok(response),
		Err(_) => Err("Could not retrieve game with game id."),
	}
}

fn update_board_gui(height: usize, board: &str, board_grid: &Grid, radio_vec: &Vec<RadioButton>) {

	// do we want to make a connection to the server to ensure that it is 
	// the one from the server ?? 
	let mut columns = vec![];
	let mut board_cp = board.clone().to_string();

	while board_cp.len() >= height {
		let a = board_cp.split_off(height);
		columns.push(board_cp);
		board_cp = a;
	}

	for col_index in 0 .. columns.len() {
		let col = columns[col_index].clone();
		println!("{}",col);
		for (row_index, c) in col.chars().enumerate() {
			println!("c: {}", c);
			println!("col_index: {}", col_index);
			println!("row_index: {}", row_index);
			println!("final product: {}", height-row_index-1);
			if c == '1' {
				let blue_piece = Image::new_from_file("blue_piece.png");
				board_grid.attach(&blue_piece, col_index as i32, (height - row_index - 1) as i32, 1, 1);
			} else if c == '2' {
				let red_piece = Image::new_from_file("red_piece.png");
				board_grid.attach(&red_piece, col_index as i32, (height - row_index - 1) as i32, 1, 1);
			}
			if row_index == height - 1 && c != '0' {
				radio_vec[col_index].set_sensitive(false);
			}
		}
	}
}

fn build_selection_game_window(game_ids: Vec<String>, ip_addr: String) {
	let select_src = include_str!("selection_window.glade");
	let selection_game_builder = Builder::new_from_string(select_src);
	let selection_window: Window = selection_game_builder.get_object("selection_window").unwrap();
	let combo_box: ComboBoxText = selection_game_builder.get_object("existing_combo").unwrap();

	//TODO: display valid games only
	for g in game_ids {
		combo_box.append_text(&g);
	}

	let join_btn: Button = selection_game_builder.get_object("join_btn").unwrap();
	join_btn.connect_clicked(move |_| {
		let active_txt = combo_box.get_active_text();
		match active_txt {
			Some(game_id) => {
				println!("{}", game_id);
				build_game_window(&game_id, Player::Two, ip_addr.clone());
			},
			None => {
				println!("User did not select game");
			},
		}
	});



	// add closure to quit application when this button is pressed
	let quit_btn: Button = selection_game_builder.get_object("cancel_btn").unwrap();
	quit_btn.connect_clicked(move |_| {
		main_quit();
    	Inhibit(false);
	});
	selection_window.show_all();
}


fn build_game_window(game_id: &str, pid: Player, ip_addr: String) {
	let game_info_res = get_game(&game_id, &ip_addr);
	if game_info_res.is_err() {
		println!("Error getting game dab");
		return;
	}
	let game_info = game_info_res.unwrap();

	//parse JSON
	let game_value: Value = from_reader(game_info).expect("Unable to parse response!");

	let k = usize::from_str(&game_value["k"].to_string()).unwrap();
	let height = usize::from_str(&game_value["height"].to_string()).unwrap();
	let width = usize::from_str(&game_value["width"].to_string()).unwrap();
	let board = &game_value["board"].to_string();
	
	let game_glade_src = include_str!("game_window.glade");
	let game_builder = Builder::new_from_string(game_glade_src);
	let game_window: Window = game_builder.get_object("game_window").unwrap();
	configure_game_window(&game_window);

	let game_board = Grid::new();
	game_board.set_name("game_grid");
	game_board.set_row_homogeneous(true);
	game_board.set_column_homogeneous(true);
	
	// set names and pictures of starting board
	for i in 0..width {
		for j in 0..height {
			let image = Image::new_from_file("empty.png");
			let mut name = i.to_string();
			name.push_str(",");
			name.push_str(&j.to_string());
			image.set_name(&name);
			game_board.attach(&image, i as i32, j as i32, 1, 1);
		}
	}

	// add radio buttons
    let mut radio_vec = vec![];
    let base = RadioButton::new_with_label_from_widget(None, "-1");
    for i in 1..width+1 {
    	let btn = RadioButton::new_with_label_from_widget(Some(&base), &i.to_string());
    	btn.set_halign(Align::Center);
    	btn.set_name(&i.to_string());
    	game_board.attach(&btn, (i-1) as i32, (height+1) as i32, 1, 1);
    	radio_vec.push(btn);
    }

	let play_button = Button::new_with_label("Play");
	play_button.set_name("play_btn");

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
 	update_board_gui(height, &board[1..board.len()-1], &game_board, &radio_vec);
	game_window.show_all();

	let g_id: usize = game_id.parse().unwrap();
	let curr_board = board.clone();
	println!("{:?}", curr_board);

	play_button.connect_clicked(move |play_btn| {
		for button in &radio_vec {
			if button.get_active() {
				let col: usize = button.get_label().unwrap().parse::<usize>().unwrap(); // get the column of move
				play_move(col, g_id, &ip_addr.clone());
				// update_board_gui(height, &board[1..board.len()-1], &game_board, &radio_vec);
				update_board_gui(height, &curr_board[1..curr_board.len()-1], &game_board, &radio_vec);
				// poll_server(board, );
				play_btn.set_sensitive(false);
				break;
			}
		}
		println!("{:?} Done playing", &pid);
	});


	let quit_btn: Button = game_builder.get_object("quit_btn").unwrap();
	quit_btn.connect_clicked(move |_| {
		main_quit();
    	Inhibit(false);
	});
}


fn connect_to_server(ip_addr: &str) -> Result<Vec<String>, &'static str> {
	let client = Client::new();
	let mut url = "http://".to_string();
	url.push_str(ip_addr);
	url.push_str("/api/connect_four.svc/Games");
	println!("{}", url);
	// let mut res_string = String::new();
    let response_result = client.get(&url).send();//.read_to_string(&mut res_string).unwrap();
    // println!("Result: {}", res_string);

    match response_result {
    	Ok(response) => {
		    if response.status == StatusCode::Ok {
			    // Parse JSON
			    let games: Vec<Value> = from_reader(response).expect("Unable to parse response!");
			    let mut game_ids = vec![];
			    for g in games {
			    	let v: Vec<usize> = g["board"].as_str().unwrap()
							    						   .chars()
							    						   .map(|x| x.to_digit(10).unwrap() as usize)
							    						   .collect();
			    	let sum: usize = v.iter().sum();
			    	if sum >= 2 { continue; }
			    	game_ids.push(g["id"].to_string());
			    }
			    Ok(game_ids)
		    } else {
		    	Err("Could not retrieve current games. Check if server is running.")
		    }
    	}
    	Err(_) => Err("Error connecting to server.")
    }

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
		if let Some(ip_addr) = ip_entry.get_text() {
			if re.is_match(&ip_addr) {
				let game_ids = connect_to_server(&ip_addr);
				match game_ids {
					Ok(g_ids) => build_selection_game_window(g_ids, ip_addr),
					Err(e) => {
						warning_label.set_text(&e);
					}
				}
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
	main();
}

