extern crate clap;
extern crate rand;

use clap::{App, Arg};
use rand::thread_rng;
use std::time::Instant;

mod deck;

fn main() {
    let matches = App::new("Deck shuffling magic")
        .about("Starts with a deck of fixed size, then starts shuffling until it matches that same deck")
        .arg(
            Arg::with_name("deck-size")
                .takes_value(true)
                .short("s")
                .long("size")
                .default_value("13")
                .help("The size of the deck to start with")
        )
        .get_matches();

    let deck_size = matches
        .value_of("deck-size")
        .unwrap_or("13")
        .parse()
        .expect("Deck size must be a number");
    let deck = deck::Deck::deal(deck_size);

    let mut shuffler = deck::Shuffler::new(deck, thread_rng());

    println!("Beginning to shuffle {} cards", shuffler.len());

    let start = Instant::now();

    loop {
        let shuffles = shuffler.shuffle();

        if shuffler.deck_matches_original() {
            println!("We did it! It only took {} shuffles", shuffles);
            break;
        } else if shuffler.shuffles() % 10_000_000 == 0 {
            println!(
                "Still shuffling...\n\
                at ~{} shuffles/sec\n\
                {}",
                shuffles / start.elapsed().as_secs() as u128,
                shuffler,
            );
        }
    }
}
