#![feature(async_closure)]

pub mod commands;
pub mod types;
pub mod user_file;
pub mod client;
pub mod cli;
pub mod tests;
pub mod util;

pub use const_format::{formatcp, concatcp};
pub use serde_derive::{Deserialize, Serialize};

pub mod consts {
    use crate::*;

    pub const MONZO_API: &'static str = "https://api.monzo.com";

    pub const REDIRECT_URI: &'static str = formatcp!("http://{}", LOCALHOST);
    pub const CLIENT_ID: &'static str = "oauth2client_0000AG6ruRfAEBcGRuXm4n";
    pub const CLIENT_SECRET: &'static str = "mnzpub.kbUehdcQ5QjR3bZ6nnOpJy50zIJw2pdnk1kWj2BbSS/RdGpBla7RYd8eoP/tq5zD0azPj450SJAzlL/6/EbuSg==";
    pub const STATE_TOKEN: &'static str = "257359a1-eccc-488d-883f-570207c8d650";

    pub const IP: [u8; 4] = [127, 0, 0, 1];
    pub const PORT: u16 = 3000;
    pub const ADDR: ([u8; 4], u16) = (IP, PORT);
    pub const LOCALHOST: &'static str =
        formatcp!("{}.{}.{}.{}:{}", IP[0], IP[1], IP[2], IP[3], PORT);
}
