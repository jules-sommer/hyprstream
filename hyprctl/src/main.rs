use hyprlib::{Hyprland, Listener};
use tokio::signal;
use tokio::time::sleep;
use tracing_subscriber::{fmt::format, prelude::*, FmtSubscriber};

#[tokio::main]
async fn main() {
  let hyprland = Hyprland::default();
  let listener = Listener::new(hyprland);
  let socket = listener.listen().await;

  let subscriber = FmtSubscriber::builder()
    .with_line_number(true)
    .event_format(format().pretty())
    .with_ansi(false)
    .finish();

  // Initialize the subscriber
  tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

  signal::ctrl_c().await.expect("failed to listen for event");
  println!("Received Ctrl-C event, shutting down...");

  if let Ok(socket) = socket {
    socket.await.unwrap();
  }
}
