use matrix_sdk::Client;
use url::Url;

fn main() {
    afl::fuzz!(|data: &[u8]| {
        if let Ok(s) = std::str::from_utf8(data) {
            if let Ok(url) = Url::parse(&s) {
                let _ = Client::new(url);
            }
        }
    });
}
