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
use std::process::exit;
use std::io::Write;
use uuid::Uuid;
use base64::decode;
use regex::Regex;
use std::str;

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
        println!("Bearer token saved to token.txt\n");
    } else {
        println!("Token not found in the response.\n");
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
    println!("This will detect if user is already registered or not.\n");

    let mut email = String::new();
    println!("Email address SSO (Single Sign On):");
    io::stdin().read_line(&mut email).expect("Failed to read input");
    let email = email.trim();

    // Read the Bearer token from the file
    let bearer_token = read_bearer_token()?;

    /* This will payload with memory input of email
       Normal, they use random deviceID from Android Device, using zeroes is anonymous ID
       Registration type is only available of traditional, later will be Google and Facebook. */

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
        .header("x-acf-sensor-data", format!("{}", sensor_data))  // Akamai sensor data
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

    let response_text = response.text()?;
    let json: Value = serde_json::from_str(&response_text)?;
    if let Some(errors) = json["status"]["errors"].as_array() {
        for error in errors {
            if let Some(code) = error["code"].as_i64() {
                if code == 41447 {
                    if let Some(error_message) = error["message"].as_str() {
                        println!("\n\x1b[91mError: {error_message}\x1b[0m\n");
                        register_australia();
                    }
                }
            }
        }
    }
    if let Some(message) = json["status"]["message"].as_str() {
        let success = format!("{message}");
        println!("\n{success}");
        australia_request(sensor_data);
    }
    println!("Response: {}", response_text);
    australia_request(sensor_data);

    Ok(())
}

fn get_uuid() -> String {
    Uuid::new_v4().as_hyphenated().to_string()
}

pub fn australia_request<A>(sensor_data: &A) -> Result<(), Box<dyn Error>>
where
    A: Display + ?Sized + Debug,
{
    println!("\nAlright, you received an email. Please open up your mail, right click where it says");
    println!("of Log in button, copy and paste the link onto terminal.");

    let mut link = String::new();
    io::stdin().read_line(&mut link).expect("Failed to read input");
    let url = link.trim();

    // Regular expression to capture the Base64 string after "ml=" and before "&"
    let re = Regex::new(r"ml=([^&]+)").expect("Invalid regex");
    let bearer_token = read_bearer_token()?;
    let mut encoded_str = None;
    
    if let Some(captures) = re.captures(url) {
        if let Some(matched_str) = captures.get(1) {
            encoded_str = Some(matched_str.as_str().to_string());
        }
    }

    let payload = json!({
        "activationlink": encoded_str,
        "clientInfo": {
            "device": {
                "deviceUniqueId": "0000000000000000",
                "os": "Android",
                "osVersion": "9"
            }
        }
    });
    
    let client = Client::new();
    let response = client
    .put("https://ap-prod.api.mcd.com/exp/v1/customer/activateandsignin")
    .header("mcd-clientid", "724uBz3ENHxUMrWH73pekFvUKvj8fD7X")
    .header(header::AUTHORIZATION, bearer_token)
    .header("x-acf-sensor-data", format!("{}", sensor_data.to_string()))  // Akamai sensor data
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

    let response_text = response.text()?;
    println!("{response_text}");

    Ok(())
}

pub fn register_australia() {
    println!("Welcome to McDonald's Australia registration!");
    println!("You may need enter new first name, last name, post code and last email address\n");
    println!("Please note, this registration is experimental of this program.\n");

    let first_name = prompt("First name");
    let last_name = prompt("Last name");
    let post_code = prompt("Post code");
    let email = prompt("Email address");

    println!("\nPlease confirm your details to register your new account:");
    println!("First name: {}", first_name);
    println!("Last name: {}", last_name);
    println!("Post code: {}", post_code);
    println!("Email address: {}", email);

    let confirmation = prompt("Type 'yes' to confirm or anything else to cancel");
    if confirmation.to_lowercase() == "yes" {
        println!("Sorry! It is unfinished, so it will be coming soon for later updates...");
        exit(1);
    } else {
        println!("Registration cancelled.");
        println!("Returning to login screen...");
        println!("Sorry! It is unfinished, so it will be coming soon for later updates...");
        exit(1);
    }
}

fn prompt(field: &str) -> String {
    print!("{}: ", field);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}