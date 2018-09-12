extern crate clap;
extern crate rand;

use clap::{App, Arg};
use rand::{thread_rng, Rng};
use std::time::Instant;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Suit {
    Club,
    Spade,
    Heart,
    Diamond,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Face {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct Card(Face, Suit);

const ALL_CARDS: [Card; 52] = [
    Card(Face::Ace, Suit::Club),
    Card(Face::Two, Suit::Club),
    Card(Face::Three, Suit::Club),
    Card(Face::Four, Suit::Club),
    Card(Face::Five, Suit::Club),
    Card(Face::Six, Suit::Club),
    Card(Face::Seven, Suit::Club),
    Card(Face::Eight, Suit::Club),
    Card(Face::Nine, Suit::Club),
    Card(Face::Ten, Suit::Club),
    Card(Face::Jack, Suit::Club),
    Card(Face::Queen, Suit::Club),
    Card(Face::King, Suit::Club),
    Card(Face::Ace, Suit::Spade),
    Card(Face::Two, Suit::Spade),
    Card(Face::Three, Suit::Spade),
    Card(Face::Four, Suit::Spade),
    Card(Face::Five, Suit::Spade),
    Card(Face::Six, Suit::Spade),
    Card(Face::Seven, Suit::Spade),
    Card(Face::Eight, Suit::Spade),
    Card(Face::Nine, Suit::Spade),
    Card(Face::Ten, Suit::Spade),
    Card(Face::Jack, Suit::Spade),
    Card(Face::Queen, Suit::Spade),
    Card(Face::King, Suit::Spade),
    Card(Face::Ace, Suit::Heart),
    Card(Face::Two, Suit::Heart),
    Card(Face::Three, Suit::Heart),
    Card(Face::Four, Suit::Heart),
    Card(Face::Five, Suit::Heart),
    Card(Face::Six, Suit::Heart),
    Card(Face::Seven, Suit::Heart),
    Card(Face::Eight, Suit::Heart),
    Card(Face::Nine, Suit::Heart),
    Card(Face::Ten, Suit::Heart),
    Card(Face::Jack, Suit::Heart),
    Card(Face::Queen, Suit::Heart),
    Card(Face::King, Suit::Heart),
    Card(Face::Ace, Suit::Diamond),
    Card(Face::Two, Suit::Diamond),
    Card(Face::Three, Suit::Diamond),
    Card(Face::Four, Suit::Diamond),
    Card(Face::Five, Suit::Diamond),
    Card(Face::Six, Suit::Diamond),
    Card(Face::Seven, Suit::Diamond),
    Card(Face::Eight, Suit::Diamond),
    Card(Face::Nine, Suit::Diamond),
    Card(Face::Ten, Suit::Diamond),
    Card(Face::Jack, Suit::Diamond),
    Card(Face::Queen, Suit::Diamond),
    Card(Face::King, Suit::Diamond),
];

#[derive(Debug, Eq, PartialEq, Clone)]
struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    fn deal(num_cards: usize) -> Deck {
        Deck {
            cards: Vec::from(&ALL_CARDS[0..num_cards]),
        }
    }

    fn len(&self) -> usize {
        self.cards.len()
    }
}

#[cfg(test)]
mod cause_i_like_tests {
    use super::*;

    #[test]
    fn decks_are_equal() {
        assert_eq!(Deck::deal(12), Deck::deal(12));
        assert_ne!(Deck::deal(13), Deck::deal(12));
    }
}

struct Shuffler<R: Rng> {
    primary: Deck,
    shuffle: Deck,
    rng: R,
    shuffles: u128,
}

impl<R: Rng> Shuffler<R> {
    fn new(deck: Deck, rng: R) -> Shuffler<R> {
        Shuffler {
            shuffle: deck.clone(),
            primary: deck,
            rng,
            shuffles: 0,
        }
    }

    fn shuffle(&mut self) {
        self.rng.shuffle(&mut self.shuffle.cards);
        self.shuffles += 1;
    }

    fn deck_matches_primary(&self) -> bool {
        self.shuffle == self.primary
    }

    fn len(&self) -> usize {
        self.primary.len()
    }
}

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
    let deck = Deck::deal(deck_size);

    let mut shuffler = Shuffler::new(deck, thread_rng());

    assert!(shuffler.deck_matches_primary());

    println!("Beginning to shuffle {} cards", shuffler.len());

    let start = Instant::now();

    loop {
        shuffler.shuffle();

        if shuffler.deck_matches_primary() {
            println!("We did it! It only took {} shuffles", shuffler.shuffles);
            break;
        } else if shuffler.shuffles % 10_000_000 == 0 {
            println!(
                "Still shuffling after {} shuffles, ~{} shuffles/sec",
                shuffler.shuffles,
                shuffler.shuffles / start.elapsed().as_secs() as u128
            );
        }
    }
}
