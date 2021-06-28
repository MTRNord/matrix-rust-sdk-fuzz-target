use std::convert::TryFrom;

use matrix_sdk::{identifiers::RoomId, Client};
use url::Url;

fn main() {
    afl::fuzz!(|data: &[u8]| {
        if let Ok(s) = std::str::from_utf8(data) {
            if let Ok(url) = Url::parse("http://127.0.0.1") {
                if let Ok(client) = Client::new(url) {
                    if let Ok(room_id) = RoomId::try_from(s) {
                        let _ = client.get_room(&room_id);
                    }
                }
            }
        }
    });
}
