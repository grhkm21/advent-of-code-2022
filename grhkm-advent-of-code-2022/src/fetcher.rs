use regex::Regex;
use reqwest::{cookie::Jar, Client, Error, Url};
use std::fs;
use std::io::{stdin, stdout, Write};
use std::process;

fn get_client(url: &str, cookie_file_path: &String) -> Client {
    let url: Url = url.parse().unwrap();

    let cookie = fs::read_to_string(cookie_file_path)
        .unwrap_or_else(|_| format!("err: can't read cookie file {cookie_file_path}"));

    let jar = Jar::default();
    jar.add_cookie_str(&cookie, &url);

    Client::builder()
        .cookie_provider(jar.into())
        .build()
        .expect("err: failed to build reqwest::Client")
}

#[tokio::main]
pub async fn fetch(day: usize, year: usize, cookie_file_path: &String) -> Result<String, Error> {
    println!("Downloading from server...");

    let url = &format!("https://adventofcode.com/{year}/day/{day}/input");
    let client = get_client(url, cookie_file_path);

    let fetch_err = format!("err: Fetching {url} failed");
    let response = client.get(url).send().await.expect(&fetch_err);

    let status = response.status();
    if !status.is_success() {
        let status_err = format!(
            "err: Fetching {url} failed with status code: {status}\nresponse: {response:#?}",
        );
        panic!("{}", &status_err);
    }

    // println!("{:?}", response);
    println!("status: {}", response.status());

    let body = response
        .text()
        .await
        .expect("err: Extracting .text() failed")
        .trim_end()
        .to_string();

    println!("Downloaded day_{day:02}.in from server");
    Ok(body)
}

#[tokio::main]
pub async fn submit(
    day: usize,
    answer: String,
    level: usize,
    year: usize,
    cookie_file_path: &String,
) {
    if level != 1 && level != 2 {
        println!("err: level = {level} is not 1 or 2!");
        process::exit(1);
    }

    // confirm from user
    print!("Submit day {day} level {level} with answer {answer} [y/N]? ");
    let _ = stdout().flush();

    let mut input = "".to_string();
    let _ = stdin().read_line(&mut input);

    if !input.to_uppercase().starts_with('Y') {
        println!("Stopping");
        return;
    }

    println!("Submitting to server...");

    let url = &format!("https://adventofcode.com/{year}/day/{day}/answer");
    let client = get_client(url, cookie_file_path);

    let params = [("answer", answer), ("level", level.to_string())];
    let response = client.post(url).form(&params).send().await;

    let response_body = if response.is_err() {
        println!("err: Submitting to {url} with params {params:?} failed");
        process::exit(1);
    } else {
        response.unwrap()
    };

    // println!("{:?}", response_body);
    println!("status: {}", response_body.status());

    let mut body = response_body
        .text()
        .await
        .expect("err: Extracting .text() failed")
        .trim()
        .to_string();

    // parse response body, wrapped in <main></main>
    let re = Regex::new(r"<main>((.|\n)*)</main>").unwrap();
    if let Some(captures) = re.captures(&body) {
        body = captures.get(1).unwrap().as_str().to_string();
    }

    // TODO: parse response and format

    println!("Submitted answer for Day #{day:02}, level {level}!");
    println!("Response body:\n{body}");
}
