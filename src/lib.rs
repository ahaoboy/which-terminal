mod detect;
mod types;
mod utils;
mod exec;

pub use types::{DetectionError, Terminal, TerminalInfo};


pub fn which_terminal() -> Option<TerminalInfo> {
     detect::detect()
}
