extern crate clap;
extern crate rand;
extern crate crossbeam_channel;

use clap::{App, Arg};
use rand::thread_rng;
use std::time::Instant;
use crossbeam_channel as channel;
use std::thread;

mod deck;

fn main() {
    let matches = App::new("Deck shuffling magic")
        .about("Starts with a deck of fixed size, then starts shuffling until it matches that same deck")
        .arg(
            Arg::with_name("deck-size")
                .takes_value(true)
                .short("s")
                .long("deck-size")
                .default_value("13")
                .help("The size of the deck to start with")
        )
        .arg(
            Arg::with_name("threads")
                .takes_value(true)
                .short("t")
                .long("threads")
                .default_value("3")
                .help("The number of threads to use for shuffling.")
        )
        .get_matches();

    let deck_size = matches
        .value_of("deck-size")
        .unwrap_or("13")
        .parse()
        .expect("Deck size must be a number");
    let deck = deck::Deck::deal(deck_size);

    let threads = matches
        .value_of("threads")
        .unwrap_or("3")
        .parse()
        .expect("A valid integer.");

    let (s, r) = channel::bounded(1024);

    println!("Beginning to shuffle {} cards on {} threads", deck.len(), threads);

    let start = Instant::now();

    for _ in 0..threads {
        let deck = deck.clone();
        let s = s.clone();
        thread::spawn(move || {
            let mut shuffler = deck::Shuffler::new(deck, thread_rng());

            loop {
                let _ = shuffler.shuffle();
                s.send(shuffler.shuffled().clone())
            }
        });
    }

    let mut shuffles = 0u128;

    while let Some(shuffled) = r.recv() {
        shuffles += 1;

        if shuffled == deck {
            println!("We did it! It only took {} shuffles", shuffles);
            break;
        } else if shuffles % 10_000_000 == 0 {
            println!(
                "{} shuffles later...\n\
                at ~{} shuffles/sec",
                shuffles,
                shuffles / start.elapsed().as_secs() as u128,
            );
        }
    }
}
