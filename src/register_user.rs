use matrix_sdk::api::r0::account::register::Request as RegistrationRequest;
use matrix_sdk::{assign, Client};
use tokio::runtime::Runtime;
use url::Url;

fn main() {
    afl::fuzz!(|data: (Option<&str>, Option<&str>)| {
        let (username, password) = data;
        if let Ok(url) = Url::parse("http://127.0.0.1:8888") {
            if let Ok(client) = Client::new(url) {
                let request = assign!(RegistrationRequest::new(), {
                    username: username,
                    password: password,
                });

                let rt = Runtime::new().unwrap();
                rt.block_on(async move {
                    let _ = client.register(request).await;
                });
            }
        }
    });
}
