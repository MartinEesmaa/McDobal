use reqwest::blocking::{Client, Response};
use reqwest::header;
use serde_json::Value;  // For JSON parsing
use std::collections::HashMap;
use std::error::Error;
use std::fs;  // For file handling
use std::env;

// Define a struct to hold the configuration
struct Config {
    url: String,
    clientid: String,
    clientsecret: String,
}

impl Config {
    fn post_request(&self) -> Result<Response, Box<dyn Error>> {
        let client = Client::new();

        // Set up the Basic Auth header
        let auth = header::HeaderValue::from_str(&format!(
            "Basic {}",
            base64::encode(format!("{}:{}", self.clientid, self.clientsecret))
        ))?;
        let mut headers = header::HeaderMap::new();
        headers.insert(header::AUTHORIZATION, auth);

        // Prepare form data as you would with `-d grantType=client_credentials`
        let mut form_data = HashMap::new();
        form_data.insert("grantType", "client_credentials");

        // Send the POST request with form data
        let response = client
            .post(&self.url)
            .headers(headers)
            .form(&form_data)  // Send form data in the request body
            .send()?;

        Ok(response)
    }
}

pub fn australia() -> Result<(), Box<dyn Error>> {
    let url = "https://ap-prod.api.mcd.com/v1/security/auth/token".to_string();
    let clientid = "724uBz3ENHxUMrWH73pekFvUKvj8fD7X".to_string();
    let clientsecret = "anr4rTy2VRaCfcr9wZE6kVKjSswTv2Rc".to_string();

    let config = Config {
        url,
        clientid,
        clientsecret,
    };

    let response = config.post_request()?;
    let response_text = response.text()?;
    let json: Value = serde_json::from_str(&response_text)?;
    
    if let Some(token) = json["response"]["token"].as_str() {  
        let bearer_token = format!("Bearer {}", token);
        
        let home_dir = env::home_dir().ok_or("Unable to find home directory")?;
        let token_dir = home_dir.join("mcdobal");

        if !token_dir.exists() {
            fs::create_dir_all(&token_dir)?; // Create mcdobal directory
        }

        let token_path = token_dir.join("token.txt");
        fs::write(&token_path, &bearer_token)?;
        println!("Bearer token saved to {:?}", token_path);
    } else {
        println!("Token not found in the response.");
    }

    Ok(())
}

pub fn usa() {
    println!("Coming soon for USA connection... just variables need to be improved");
}