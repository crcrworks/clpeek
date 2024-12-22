use colored::*;
use eyre::{bail, Result};
use rand::Rng;
use std::{fmt::Display, io::Write as _};

/// Deletes one line from the terminal output.
///
/// This function moves the cursor up one line and then clears the entire line.
/// It uses ANSI escape codes to achieve this:
/// - `\x1b[F` moves the cursor up one line.
/// - `\x1b[2K` clears the entire line.
fn delete_one_line() {
    print!("\x1b[F");
    print!("\x1b[2K");
}

#[derive(Default)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Color {
    fn generate_random() -> Self {
        let mut rng = rand::thread_rng();

        let red = rng.gen_range(0..255);
        let green = rng.gen_range(0..255);
        let blue = rng.gen_range(0..255);

        Color { red, green, blue }
    }

    /// Calculates the accuracy of the color match.
    /// Returns a value in the range from 0 to 100, where 100 means a perfect match.
    fn calc_accuracy(&self, answer: &Color) -> f64 {
        let red_diff = (self.red as i32 - answer.red as i32).abs() as f64;
        let green_diff = (self.green as i32 - answer.green as i32).abs() as f64;
        let blue_diff = (self.blue as i32 - answer.blue as i32).abs() as f64;

        let total_diff = red_diff + green_diff + blue_diff;
        let max_diff = 3.0 * 255.0;

        100.0 - (total_diff / max_diff * 100.0)
    }
}

impl TryFrom<&str> for Color {
    type Error = eyre::Error;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        let hex = value.trim_start_matches('#');

        if hex.len() != 6 {
            bail!("input exactly 6: {}", value);
        }

        let red = u8::from_str_radix(&hex[0..2], 16)?;
        let green = u8::from_str_radix(&hex[2..4], 16)?;
        let blue = u8::from_str_radix(&hex[4..6], 16)?;

        Ok(Color { red, green, blue })
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let color_string = " ".repeat(20);

        write!(
            f,
            "{}",
            color_string.on_truecolor(self.red, self.green, self.blue)
        )
    }
}

fn main() -> Result<()> {
    println!("Guess this color:");

    let generated_color = Color::generate_random();

    println!(" ---------------------- ");
    println!("| {} |", generated_color);
    println!(" ---------------------- ");

    println!("Aim for an accuracy above 90%!");
    println!();

    loop {
        let (input, guessed_color) = loop {
            print!("type color:");
            std::io::stdout().flush().unwrap();

            let mut input = String::new();
            if let Err(e) = std::io::stdin().read_line(&mut input) {
                println!("{e}");
                continue;
            }

            delete_one_line();

            let input = String::from(input.trim());

            let guessed_color = match input.trim().try_into() {
                Ok(v) => v,
                Err(e) => {
                    println!("{e}");
                    continue;
                }
            };

            break (input, guessed_color);
        };

        let accuracy = generated_color.calc_accuracy(&guessed_color);

        println!("{:.2}% | {} {}", accuracy, input, guessed_color);

        if 90. < accuracy {
            println!("You did it!");
            break;
        }
    }

    Ok(())
}
