# connect_four_gui

Generates the GUI for the client-side executable of the the [ConnectK](https://github.com/eecs395rust/ConnectFour) game implementation

Functionality of GUI:
* **poll_server**: polls server continuously to check whether other player has made a move
* **play_move**: plays the move specified by the current user
* **get_game**: gets game with specific game id from [game server](https://github.com/mmgeorge/game_server)
* **update_board_gui**: updates the board GUI after a move has been made
* **build_selection_game_window**: creates the parts of the GUI that allows user to select a game (only games that have less than two players can be joined) and create a game
* **build_game_window**: builds game window with a ConnectK board and Play Move button
* **connect_to_server**: connects to [game server](https://github.com/mmgeorge/game_server) to get information about current joinable games & how to create new game
* **launch**: launches GUI, validates IP address of entered server


# Dependencies 
* gtk = { git = "https://github.com/gtk-rs/gtk.git", features = ["v3_16"] }
* gdk = { git = "https://github.com/gtk-rs/gdk.git" }
* regex = "0.2"         
* hyper = "0.10"           
* serde_json = "1.0"       
* serde_derive = "1.0.4"   
* serde = "1.0.4" 