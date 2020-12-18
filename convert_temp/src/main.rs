use std::io;

fn main() {
    loop {
        println!("Temperature in Fahrenheit ?");

        let mut fahrenheit = String::new();

        io::stdin()
            .read_line(&mut fahrenheit)
            .expect("Failed to read line");

        let fahrenheit: u32 = match fahrenheit.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let celsius = (fahrenheit - 32) * 5 / 9;

        println!("{}° F is {}° C", fahrenheit, celsius);
    }
}
