use reqwest::{cookie::Jar, Client, Error, Url};
use std::fs;

#[tokio::main]
pub async fn fetch(day: usize, year: usize) -> Result<String, Error> {
    let cookie = fs::read_to_string("./cookies").expect("err: can't read file");
    println!("Downloading from server...");

    let url: Url = format!("https://adventofcode.com/{}/day/{}/input", year, day)
        .parse()
        .unwrap();
    let url_str = &format!("{}", url);

    let jar = Jar::default();
    jar.add_cookie_str(&cookie, &url);

    let client = Client::builder()
        .cookie_provider(jar.into())
        .build()
        .expect("err: failed to build reqwest::Client");

    let fetch_err = format!("err: Fetching {} failed", url_str);
    let response = client.get(url).send().await.expect(&fetch_err);

    let status = response.status();
    if !status.is_success() {
        let status_err = format!(
            "err: Fetching {} failed with status code: {}\nresponse: {:#?}",
            url_str, status, response
        );
        panic!("{}", &status_err);
    }

    println!("{:?}", response);
    println!("status: {}", response.status());

    let body = response
        .text()
        .await
        .expect("err: Extracting .text() failed");

    println!("Downloaded day_{:02}.in from server", day);
    Ok(body)
}
