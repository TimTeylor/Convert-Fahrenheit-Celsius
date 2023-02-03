use std::io;

fn main() {
    loop {
        println!("Please input the fahrenheit");

        let mut fahrenheit = String::new();

        if let Ok(_) = io::stdin().read_line(&mut fahrenheit) {
            if let Ok(fahrenheit) = fahrenheit.trim().parse::<f64>() {
                let celsius = (fahrenheit - 32.0) / 1.8000;
                println!("You degrees Celsius {}", celsius);
            } else {
                println!("Invalid input, please enter a valid number");
            }
        } else {
            println!("Failed to read input");
        }
    }
}
