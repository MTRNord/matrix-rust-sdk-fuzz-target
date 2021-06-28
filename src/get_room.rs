use std::convert::TryFrom;

use matrix_sdk::{identifiers::RoomId, Client};
use url::Url;

fn main() {
    afl::fuzz!(|data: &str| {
        if let Ok(url) = Url::parse("http://127.0.0.1") {
            if let Ok(client) = Client::new(url) {
                if let Ok(room_id) = RoomId::try_from(data) {
                    let _ = client.get_room(&room_id);
                }
            }
        }
    });
}
