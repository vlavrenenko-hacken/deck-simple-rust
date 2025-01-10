use rand::{seq::SliceRandom, thread_rng};
trait DeckFunctionality {
    fn new() -> Deck;
    fn shuffle(&mut self);
    fn deal(&mut self, number_of_cards_to_delete: usize) -> Vec<String>;
}

#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

impl DeckFunctionality for Deck {
    fn new() -> Deck {
        let suits = ["Spades", "Hearts", "Diamonds", "Clubs", "Jokers"];
        let values = ["Ace", "Two", "Three"];
        let mut cards = vec![];

        for suit in suits {
            for value in values {
                let card = format!("{} of {} ", value, suit);
                cards.push(card);
            }
        }

        Deck { cards: cards }
    }
    fn shuffle(&mut self) {
        let mut _rng = thread_rng();
        self.cards.shuffle(&mut _rng);
    }
    fn deal(&mut self, number_of_cards_to_delete: usize) -> Vec<String> {
        self.cards
            .split_off(self.cards.len() - number_of_cards_to_delete)
    }
}

fn main() {
    // new() -> Creates a new deck object that contains a list of playing cards
    // shuffle() -> Shuffles the order of cards in this deck
    // deal() -> Removes some playing cards from the deck and returns them in a new list

    let mut deck = Deck::new();
    //deck.shuffle();
    let _deck = deck.deal(3);
    println!("Here is your hand: {:#?}", _deck);
    println!("Remaining cards: {:?}", deck);
}
