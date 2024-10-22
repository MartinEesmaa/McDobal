use text2art::BasicFonts;
use text2art::Font;
use text2art::Printer;
use std::io;

mod connect;

enum Choice {
    AU,
    CA,
    DE,
    JP,
    LA,
    MA,
    PH,
    UK,
    USA,
    WW,
    Unsupported,
    Invalid,
}

fn main() {
    let version: &str = "0.0.1";

    let font = match Font::from_basic(BasicFonts::Big) {
        Ok(font) => font,
        Err(_) => panic!("Error rendering font"),
    };

    let prntr = Printer::with_font(font);
    prntr.print_to_stdio("McDobal").ok();
    println!("\nVersion {version} (Prototype)");
    println!("Created by Martin Eesmaa (2024)\n");
    println!("This project is experimental and prototype. Coming soon...");

    loop {
        println!("Please choose to connect:\n");
        println!("1. Australia");
        println!("2. Canada");
        println!("3. Germany");
        println!("4. Japan");
        println!("5. Latin America");
        println!("6. Mesoamerica");
        println!("7. Philippines");
        println!("8. United Kingdom");
        println!("9. United States");
        println!("10. Worldwide");
        println!("Number:");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        let choice = match input.trim() {
            "1" => Choice::AU,
            "2" => Choice::Unsupported,
            "3" => Choice::Unsupported,
            "4" => Choice::Unsupported,
            "5" => Choice::Unsupported,
            "6" => Choice::Unsupported,
            "7" => Choice::Unsupported,
            "8" => Choice::Unsupported,
            "9" => Choice::USA,
            "10" => Choice::Unsupported,
            _ => Choice::Invalid,
        };

        match choice {
            Choice::AU => {
                println!("Choosen Australia, connecting...");
                connect::australia();
            } 
            Choice::USA => {
                println!("Choosen USA, connecting...");
                connect::usa();
            }
            Choice::Invalid => {
                println!("Invalid choice, please try again.");
                continue;
            },
            Choice::Unsupported => println!("Sorry... unsupported yet. Coming soon..."),
            _ => todo!()
        }

        break;
    }
}
