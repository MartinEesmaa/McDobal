use reqwest::blocking::{Client, Response};
use reqwest::header;
use serde_json::Value;  // For JSON parsing
use serde_json::json;
use std::collections::HashMap;
use std::error::Error;
use std::fs;  // For file handling
use std::path::Path;
use std::io;
use std::fmt::{Debug, Display};
use uuid::Uuid;

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

    // Initialize the config
    let config = Config {
        url,
        clientid,
        clientsecret,
    };

    // Make the POST request
    let response = config.post_request()?;

    // Handle the response
    let response_text = response.text()?;
    
    // Parse the response JSON and extract the token
    let json: Value = serde_json::from_str(&response_text)?;
    if let Some(token) = json["response"]["token"].as_str() {
        let bearer_token = format!("Bearer {}", token);
        
        // Save the bearer token to a file
        let dir = Path::new("mcdobal");
        if !dir.exists() {
            fs::create_dir(&dir)?;
        }
        fs::write(dir.join("token.txt"), &bearer_token)?;
        println!("Bearer token saved to token.txt");
    } else {
        println!("Token not found in the response.");
    }
    email_australia("&a")?;
    Ok(())
}

pub fn usa() {
    println!("Coming soon for USA connection... just variables need to be improved");
}

// Function to read the Bearer token from the file
fn read_bearer_token() -> Result<String, Box<dyn Error>> {
    let token = fs::read_to_string("mcdobal/token.txt")?;
    Ok(token.trim().to_string())
}

// New function to handle email registration/login
pub fn email_australia<A>(sensor_data: &A) -> Result<(), Box<dyn Error>>
where
    A: Display + ?Sized + Debug,
{
    println!("Welcome to McDonald's Australia, please enter your email address to log in or register");

    let mut email = String::new();
    io::stdin().read_line(&mut email).expect("Failed to read input");
    let email = email.trim();

    // Read the Bearer token from the file
    let bearer_token = read_bearer_token()?;

    let payload = json!({
        "customerIdentifier": email,
        "deviceId": "0000000000000000",
        "registrationType": "traditional",
    });

    let client = Client::new();
    let response = client
        .post("https://ap-prod.api.mcd.com/exp/v1/customer/identity/email")
        .header("mcd-clientid", "724uBz3ENHxUMrWH73pekFvUKvj8fD7X")
        .header(header::AUTHORIZATION, bearer_token)
        .header("x-acf-sensor-data", format!("{}", sensor_data))  // Pass sensor data as header value
        .header("user-agent", "")
        .header("mcd-sourceapp", "GMA")
        .header("mcd-marketid", "au")
        .header("accept-encoding", "gzip")
        .header("accept-charset", "UTF-8")
        .header("accept-language", "en-AU")
        .header("content-type", "application/json; charset=UTF-8")
        .header("mcd-uuid", get_uuid())
        .json(&payload)
        .send()?;

    // Handle the response
    let response_text = response.text()?;
    println!("Response: {}", response_text);

    Ok(())
}

fn get_uuid() -> String {
    Uuid::new_v4().as_hyphenated().to_string()
}
