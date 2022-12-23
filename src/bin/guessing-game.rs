use std::cmp::Ordering;

use inquire::{validator::Validation, CustomType};
use rand::{thread_rng, Rng};

type Guess = u32;

fn main() {
    let max = ask_range();

    let target = thread_rng().gen_range(0..=max);

    guessing_game(&target, &max);
}

fn ask_range() -> Guess {
    // What's the range? Default: 10
    let max = CustomType::<Guess>::new("What's the range?")
        .with_default(10)
        .with_error_message("Please type a positive integer")
        .prompt();

    match max {
        Ok(v) => v,
        Err(_) => panic!(),
    }
}

fn guessing_game(target: &Guess, max: &Guess) {
    let mut guess_count = 0;

    let m_max = *max;

    loop {
        let guess = CustomType::<Guess>::new("Guess a number!")
            .with_validator(move |v: &Guess| {
                if *v > m_max {
                    Ok(Validation::Invalid("That's too high!".into()))
                } else {
                    Ok(Validation::Valid)
                }
            })
            .with_formatter(&|g: Guess| match g.cmp(target) {
                Ordering::Less => format!("{} is too low", g),
                Ordering::Greater => format!("{} is too high", g),
                Ordering::Equal => format!("Yes! {} is the answer!", g),
            })
            .prompt();

        guess_count += 1;

        match guess {
            Ok(v) if v == *target => break,
            Ok(_) => continue,
            Err(_) => panic!(),
        }
    }

    println!("It took you {} guesses!", guess_count);
}
