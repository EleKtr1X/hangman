use std::{io::{self, Write}, thread::sleep, time::Duration};
use reqwest::{self, Error};
use itertools::Itertools;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let word = &reqwest::get("https://random-word-api.herokuapp.com/word?swear=0")
        .await?
        .json::<Vec<String>>()
        .await?[0];

    let mut guess_word = vec!["_".to_owned(); word.len()];

    let mut errors = vec![];

    while errors.len() < 6 {
        if guess_word.join("").chars().all(char::is_alphabetic) {
            break;
        };
        draw_diagram(&errors);
        println!("\n\n  {}", guess_word.join(" "));
        print!("\nEnter your guess: ");
        io::stdout().flush().unwrap();
        
        let mut guess = String::new();
        
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        guess = guess.trim().to_lowercase().to_string();

        if guess.len() != 1 || !guess.chars().all(char::is_alphabetic) {
            println!("Invalid guess!");
            sleep(Duration::from_millis(2000));
            continue;
        }

        if word.contains(&guess) {
            for (pos, ch) in word.chars().enumerate() {
                if ch.to_string() == guess {
                    guess_word[pos] = guess.to_owned();
                }
            }
        } else {
            errors.push(guess);
            errors = errors.into_iter().unique().collect();
        }
    }

    draw_diagram(&errors);

    if errors.len() == 6 { println!("\nYou lost!\nThe word was: {}", word); }
    else {
        println!("\n\n  {}", guess_word.join(" "));
        println!("\nYou won!");
    }

    Ok(())
}

fn draw_diagram(errors: &Vec<String>) {
    let len = errors.len();
    println!("{esc}c", esc = 27 as char);

    println!("       Hangman");
    println!("     ___________");
    println!("     |         |");
    println!("     {}         |         Misses:   ", if len >= 1 { "O" } else { " " });
    println!("    {}{}{}        |         {}",
        if len >= 3 { "/" } else { " " },
        if len >= 2 { "|" } else { " " },
        if len >= 4 { "\\" } else { " " },
        errors.join(", "));
    println!("    {} {}        |",
        if len >= 5 { "/" } else { " " },
        if len >= 6 { "\\" } else { " " });
    println!("               |");
    println!("   ____________|__");

}