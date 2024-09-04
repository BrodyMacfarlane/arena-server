pub mod config;

use bevy::prelude::*;
use config::{get_config, Config};
use tokio::time::{interval, Duration};
use tokio::net::UdpSocket;

struct GameState {
    // Your game state here
}

fn server_address() -> String {
    format!("0.0.0.0:{}", get_config().server.port)
}

#[tokio::main]
async fn main() {
    let config: &Config = get_config();

    println!("{:#?}", config);

    let mut app = App::new();
    
    // Add your Bevy systems and plugins here
    app.add_plugins(MinimalPlugins)
       .add_systems(Update, game_logic_system);

    // Initialize your game state
    let mut game_state = GameState { /* ... */ };

    // Create a UDP socket
    let socket = UdpSocket::bind(server_address()).await.unwrap();

    // Create a tick interval
    let tick_duration = Duration::from_secs_f64(1.0 / config.server.tick_rate as f64);
    let mut tick_interval = interval(tick_duration);

    loop {
        // Wait for the next tick
        tick_interval.tick().await;

        // Handle incoming UDP messages
        handle_network_messages(&socket, &mut game_state).await;

        // Update game state
        app.update();

        // Send updates to clients
        send_updates_to_clients(&socket, &game_state).await;
    }
}

fn game_logic_system(world: &mut World) {
    // Your game logic here
}

async fn handle_network_messages(socket: &UdpSocket, game_state: &mut GameState) {
    // Handle incoming UDP messages
}

async fn send_updates_to_clients(socket: &UdpSocket, game_state: &GameState) {
    // Send game state updates to clients
}
