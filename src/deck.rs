use rand::Rng;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Suit {
    Club,
    Spade,
    Heart,
    Diamond,
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let c = match self {
            Suit::Club => "C",
            Suit::Spade => "S",
            Suit::Heart => "H",
            Suit::Diamond => "D",
        };
        write!(f, "{}", c)
    }
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

impl fmt::Display for Face {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let c = match self {
            Face::Ace => "A",
            Face::Two => "2",
            Face::Three => "3",
            Face::Four => "4",
            Face::Five => "5",
            Face::Six => "6",
            Face::Seven => "7",
            Face::Eight => "8",
            Face::Nine => "9",
            Face::Ten => "T",
            Face::Jack => "J",
            Face::Queen => "Q",
            Face::King => "K",
        };
        write!(f, "{}", c)
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct Card(Suit, Face);

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.0, self.1)
    }
}

const ALL_CARDS: [Card; 52] = [
    Card(Suit::Club, Face::Ace),
    Card(Suit::Club, Face::Two),
    Card(Suit::Club, Face::Three),
    Card(Suit::Club, Face::Four),
    Card(Suit::Club, Face::Five),
    Card(Suit::Club, Face::Six),
    Card(Suit::Club, Face::Seven),
    Card(Suit::Club, Face::Eight),
    Card(Suit::Club, Face::Nine),
    Card(Suit::Club, Face::Ten),
    Card(Suit::Club, Face::Jack),
    Card(Suit::Club, Face::Queen),
    Card(Suit::Club, Face::King),
    Card(Suit::Spade, Face::Ace),
    Card(Suit::Spade, Face::Two),
    Card(Suit::Spade, Face::Three),
    Card(Suit::Spade, Face::Four),
    Card(Suit::Spade, Face::Five),
    Card(Suit::Spade, Face::Six),
    Card(Suit::Spade, Face::Seven),
    Card(Suit::Spade, Face::Eight),
    Card(Suit::Spade, Face::Nine),
    Card(Suit::Spade, Face::Ten),
    Card(Suit::Spade, Face::Jack),
    Card(Suit::Spade, Face::Queen),
    Card(Suit::Spade, Face::King),
    Card(Suit::Heart, Face::Ace),
    Card(Suit::Heart, Face::Two),
    Card(Suit::Heart, Face::Three),
    Card(Suit::Heart, Face::Four),
    Card(Suit::Heart, Face::Five),
    Card(Suit::Heart, Face::Six),
    Card(Suit::Heart, Face::Seven),
    Card(Suit::Heart, Face::Eight),
    Card(Suit::Heart, Face::Nine),
    Card(Suit::Heart, Face::Ten),
    Card(Suit::Heart, Face::Jack),
    Card(Suit::Heart, Face::Queen),
    Card(Suit::Heart, Face::King),
    Card(Suit::Diamond, Face::Ace),
    Card(Suit::Diamond, Face::Two),
    Card(Suit::Diamond, Face::Three),
    Card(Suit::Diamond, Face::Four),
    Card(Suit::Diamond, Face::Five),
    Card(Suit::Diamond, Face::Six),
    Card(Suit::Diamond, Face::Seven),
    Card(Suit::Diamond, Face::Eight),
    Card(Suit::Diamond, Face::Nine),
    Card(Suit::Diamond, Face::Ten),
    Card(Suit::Diamond, Face::Jack),
    Card(Suit::Diamond, Face::Queen),
    Card(Suit::Diamond, Face::King),
];

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn deal(num_cards: usize) -> Deck {
        let num_cards = if num_cards > 52 { 52 } else { num_cards };
        Deck {
            cards: Vec::from(&ALL_CARDS[0..num_cards]),
        }
    }

    pub fn len(&self) -> usize {
        self.cards.len()
    }
}

impl fmt::Display for Deck {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let cards = self
            .cards
            .iter()
            .map(|c| format!("{}", c))
            .collect::<Vec<String>>()
            .as_slice()
            .join(", ");
        write!(f, "[{}]", &cards)
    }
}

#[cfg(test)]
mod deck_tests {
    use super::*;

    #[test]
    fn decks_are_equal() {
        assert_eq!(Deck::deal(12), Deck::deal(12));
        assert_ne!(Deck::deal(13), Deck::deal(12));
    }

    #[test]
    fn decks_can_be_displayed() {
        assert_eq!(format!("{}", Deck::deal(4)), "[CA, C2, C3, C4]")
    }
}

pub struct Shuffler<R: Rng> {
    original: Deck,
    shuffled: Deck,
    rng: R,
}

impl<R: Rng> Shuffler<R> {
    pub fn new(deck: Deck, rng: R) -> Shuffler<R> {
        Shuffler {
            shuffled: deck.clone(),
            original: deck,
            rng,
        }
    }

    pub fn shuffled(&self) -> &Deck {
        &self.shuffled
    }

    pub fn shuffle(&mut self) {
        self.rng.shuffle(&mut self.shuffled.cards);
    }
}

impl<R: Rng> fmt::Display for Shuffler<R> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "cards:    {}", self.original)?;
        writeln!(f, "shuffled: {}", self.shuffled)?;
        Ok(())
    }
}

#[cfg(test)]
mod shuffler_tests {
    use super::*;
    use rand::thread_rng;

    #[test]
    fn displays_self() {
        let shuffler = Shuffler::new(Deck::deal(3), thread_rng());
        assert_eq!(
            format!("{}", shuffler),
            "cards:    [CA, C2, C3]\n\
             shuffled: [CA, C2, C3]\n"
        );
    }

    #[test]
    fn shuffles_the_deck() {
        let mut shuffler = Shuffler::new(Deck::deal(52), thread_rng());
        assert_eq!(shuffler.shuffled, shuffler.original);
        shuffler.shuffle(); // The odds of this matching are astronomical.
        assert_ne!(shuffler.shuffled, shuffler.original);
    }
}
