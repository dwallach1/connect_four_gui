# ConnectK GUI

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


# Workflow

This repository holds the client-side GUI for our ConnectK Rust package. 

* It begins by prompting the user to enter the IP address of the server. This IP address, for the purposes of this project, is either a local host (i.e. http://127.0.0.1:8080) or a wildcat server exposing a local host via Ngrok (i.e. http://4f315d8e.ngrok.io). The IP address is configured by the [game server](https://github.com/mmgeorge/game_server) program which is the server-side executable. 
* If given a valid IP address, validated by a connection to the running server, the client is then presented with a screen to either join exisitng games or create a new game with W columns, H rows and an amount of K in succession needed to win the game. For the game to be valid, K must be less than or equal to either H or W. 
* After this, the main game screen is brought to view. If it is the user's turn, all the buttons will be enabled (unless a column is full) and the user will be able to select a radio button and then click play to send their move. This will cause their game state to begin polling the server for the other players move. Polling disables the buttons on their screen as they should not be able to play moves when it is not their turn. The game screen will also poll when you join a game and it is not your turn. 
* After a game is over (either won, tied or abandoned) the sceen will display a message and the user can exit out of the screen and either start/join a new game or exit the ConnectK program. 

# Dependencies 
* gtk = { git = "https://github.com/gtk-rs/gtk.git", features = ["v3_16"] }
* gdk = { git = "https://github.com/gtk-rs/gdk.git" }
* regex = "0.2"         
* hyper = "0.10"           
* serde_json = "1.0"       
* serde_derive = "1.0.4"   
* serde = "1.0.4" 

