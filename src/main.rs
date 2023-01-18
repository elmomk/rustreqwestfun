use std::collections::HashMap;
// use reqwest::blocking::{get, Response};
use reqwest::blocking::get;
// use serde_json::Result;


fn gethttp(url: String) -> HashMap<String, HashMap<String, String>> {
    get(url).expect("Failed to get response")
        // .json::<HashMap<String, String>>()?;
        .json::<HashMap<String, HashMap<String, String>>>().unwrap()
    // println!("{:#?}", resp);
    // return resp;
}

// fn posthttp(url: String) -> Result<(), Box<dyn std::error::Error>> {
//     let client = reqwest::blocking::Client::new();
//     let resp = client.post(url)
//         .json(r#"{
//             "name": "John Doe",
//             "age": 43,
//             "phones": [
//                 "+44 1234567",
//                 "+44 2345678"
//             ]
//         }"#)
//         .send()?
//         .json::<HashMap<String, HashMap<String, String>>>()?;
//     println!("{:#?}", resp);
//     Ok(())
// }



fn main() {
    let output = gethttp("https://httpbin.org/headers".to_string());
    println!("{:#?}", output);
    // let _ = posthttp("https://httpbin.org/status/101".to_string());
}
