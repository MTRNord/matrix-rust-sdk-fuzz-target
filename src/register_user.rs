use matrix_sdk::api::r0::account::register::Request as RegistrationRequest;
use matrix_sdk::api::r0::uiaa::AuthData;
use matrix_sdk::{assign, Client};
use tokio::runtime::Runtime;
use url::Url;

fn main() {
    afl::fuzz!(|data: (&str, &str, &str)| {
        let (username, password, session) = data;
        if let Ok(url) = Url::parse("http://127.0.0.1:8888") {
            if let Ok(client) = Client::new(url) {
                let request = assign!(RegistrationRequest::new(), {
                    username: Some(username),
                    password: Some(password),
                    auth: Some(AuthData::FallbackAcknowledgement { session }),
                });

                let rt = Runtime::new().unwrap();
                rt.block_on(async move {
                    let _ = client.register(request).await;
                });
            }
        }
    });
}
