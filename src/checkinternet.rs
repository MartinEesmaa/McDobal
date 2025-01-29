use std::process::exit;
use ureq;

pub fn main() {
    let test_url = "https://example.com";
    if ureq::get(test_url).call().is_err() {
        println!("\n\x1b[91mNo internet connection, make sure internet is connected.\x1b[0m");
        println!("\x1b[91mAlso mobile data can be out of signal or data, try get better location or recharge data.\x1b[0m");
        exit(1);
    }
}
