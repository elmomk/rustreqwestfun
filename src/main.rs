use std::collections::HashMap;
use reqwest::blocking::get;
// use serde_json::Result;


fn gethttp(url: String) -> Result<(), Box<dyn std::error::Error>> {
// let url = "https://httpbin.org/headers";
    let resp = get(url)?
        // .json::<HashMap<String, String>>()?;
        .json::<HashMap<String, HashMap<String, String>>>()?;
    println!("{:#?}", resp);
    Ok(())
}

fn posthttp(url: String) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    let resp = client.post(url)
        .json(r#"{
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#)
        .send()?
        .json::<HashMap<String, HashMap<String, String>>>()?;
    println!("{:#?}", resp);
    Ok(())
}



fn main() {
    // let _ = gethttp("https://httpbin.org/headers".to_string());
    let _ = posthttp("https://httpbin.org/status/101".to_string());
}
