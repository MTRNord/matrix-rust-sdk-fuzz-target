use matrix_sdk::Client;
use url::Url;

fn main() {
    afl::fuzz!(|data: &str| {
        if let Ok(url) = Url::parse(&data) {
            let _ = Client::new(url);
        }
    });
}
