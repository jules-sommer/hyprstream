mod events;
mod interpreter;

use crate::interpreter::Interpreter;
use tokio::net::UnixStream;

use tokio::io::{self, AsyncBufReadExt, BufReader};
use tracing::{error, event, info, span, Level};

#[derive(Clone, Debug, PartialEq)]
pub struct Hyprland {
  pub instance_id: String,
}

impl Hyprland {
  pub fn new(instance_id: String) -> Self {
    Hyprland { instance_id }
  }

  pub fn default() -> Self {
    Hyprland {
      instance_id: std::env::var("HYPRLAND_INSTANCE_SIGNATURE").unwrap(),
    }
  }
}

enum Socket {
  Listener,
  Dispatcher,
}

pub struct Dispatcher {
  pub hyprland: Hyprland,
}

pub struct Listener {
  pub hyprland: Hyprland,
}

impl Listener {
  pub fn new(hyprland: Hyprland) -> Self {
    Listener { hyprland }
  }

  pub async fn listen(&self) -> io::Result<tokio::task::JoinHandle<()>> {
    let socket_path = self.get_socket_path(Socket::Listener);
    let stream = UnixStream::connect(&socket_path).await?;

    // Wrap the stream in a BufReader, which provides the lines method
    let reader = BufReader::new(stream);

    let mut num_lines = 0;

    let listener_handle = tokio::spawn(async move {
      let mut lines = reader.lines();
      while let Ok(line) = lines.next_line().await {
        num_lines += 1;
        match line {
          Some(line) => {
            let event = Interpreter::new(line.as_str()).interpret();
            match event {
              Ok(event) => {
                info!(num_lines = ?num_lines, event = ?event, "Event received");
              }
              Err(e) => {
                error!("Error: {}", e);
              }
            }
          }
          None => {
            error!("End of file...");
            break;
          }
        }
      }
    });

    Ok(listener_handle)
  }

  fn get_socket_path(&self, socket_type: Socket) -> String {
    match socket_type {
      Socket::Listener => format!("/tmp/hypr/{}/.socket2.sock", self.hyprland.instance_id),
      Socket::Dispatcher => format!("/tmp/hypr/{}/.socket.sock", self.hyprland.instance_id),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::thread::sleep;

  #[test]
  fn init() {
    let hyprland = Hyprland::default();
    assert!(hyprland.instance_id.len() > 0);
  }
}

mod async_tests {
  use super::*;
  use tokio::time::{sleep, Duration};

  #[tokio::test]
  async fn async_listen() {
    let hyprland = Hyprland::default();
    let listener = Listener::new(hyprland);
    listener.listen().await.unwrap();

    sleep(Duration::from_secs(5)).await;
  }
}
