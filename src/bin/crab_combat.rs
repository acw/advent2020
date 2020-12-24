use advent2020::errors::TopLevelError;
use std::env;
use std::fmt;
use std::fs;
use std::iter::FromIterator;
use std::str::FromStr;
use std::{
    collections::{HashMap, VecDeque},
    sync::atomic::{AtomicUsize, Ordering},
};

#[derive(Clone, Eq, Hash, PartialEq)]
struct Deck {
    player: usize,
    cards: VecDeque<usize>,
}

impl Deck {
    fn read<'a, I: Iterator<Item = &'a str>>(lines: &mut I) -> Result<Deck, TopLevelError> {
        match lines.next() {
            None => Err(TopLevelError::NoInputFound),
            Some("") => Deck::read(lines),
            Some(x) if x.starts_with("Player ") => {
                let numeric_string = x.trim_start_matches("Player ").trim_end_matches(':');
                let player = usize::from_str(numeric_string)?;
                let mut cards = VecDeque::new();

                loop {
                    match lines.next() {
                        None => break,
                        Some("") => break,
                        Some(x) => cards.push_back(usize::from_str(x)?),
                    }
                }

                Ok(Deck { player, cards })
            }
            Some(_) => Err(TopLevelError::UnknownError),
        }
    }

    fn top(&mut self) -> Option<usize> {
        self.cards.pop_front()
    }

    fn add_cards(&mut self, winning_card: usize, cards: &mut Vec<usize>) {
        cards.sort_unstable();
        cards.reverse();
        self.cards.push_back(winning_card);
        for card in cards.drain(..).filter(|x| *x != winning_card) {
            self.cards.push_back(card);
        }
    }

    fn size(&self) -> usize {
        self.cards.len()
    }

    fn resize(&mut self, new_size: usize) {
        self.cards.truncate(new_size);
    }
}

impl fmt::Display for Deck {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Player {}'s deck: {:?}", self.player, self.cards)
    }
}

struct Game {
    id: usize,
    round: usize,
    history: Vec<HashMap<usize, Deck>>,
    decks: HashMap<usize, Deck>,
}

static NEXT_GAME_NUMBER: AtomicUsize = AtomicUsize::new(1);

impl Game {
    fn new(decks: &[Deck]) -> Game {
        Game {
            id: NEXT_GAME_NUMBER.fetch_add(1, Ordering::SeqCst),
            history: Vec::new(),
            round: 0,
            decks: HashMap::from_iter(decks.iter().map(|x| (x.player, x.clone()))),
        }
    }

    fn winner(&self) -> Option<&Deck> {
        let mut result = None;

        for deck in self.decks.values() {
            if deck.size() > 0 && result.is_none() {
                result = Some(deck);
            } else if deck.size() > 0 {
                return None;
            }
        }

        result
    }

    fn play(&mut self, recursive: bool) -> Result<Deck, TopLevelError> {
        loop {
            // first, see if we're done
            if let Some(winner) = self.winner() {
                return Ok(winner.clone());
            }

            // OK, let's print out some state information
            self.round += 1;
            println!(
                "-- Game {}, Round {} {}--",
                self.id,
                self.round,
                if recursive { "[recursive]" } else { "" }
            );
            for deck in self.decks.values() {
                println!("{}", deck);
            }

            // if this is the recursive version of the game and we've been here before,
            // just stop, and player 1 won.
            if recursive && self.history.contains(&self.decks) {
                return self
                    .decks
                    .get(&1)
                    .cloned()
                    .ok_or(TopLevelError::UnknownError);
            }

            if recursive {
                self.history.push(self.decks.clone());
            }

            // otherwise, grab the first card off each of the decks
            let mut top_card_info = Vec::new();

            for (player, deck) in self.decks.iter_mut() {
                if let Some(top_card) = deck.top() {
                    let recurse_check_value = deck.size() >= top_card;
                    println!(
                        "Top card for Player {}: {} [recurse check: {}]",
                        player, top_card, recurse_check_value
                    );
                    top_card_info.push((deck, top_card, recurse_check_value));
                }
            }

            let mut top_cards = top_card_info.iter().map(|(_, x, _)| *x).collect();

            // if we're in a recursive game, and we meet the length conditions, recurse
            if recursive && top_card_info.iter().all(|(_, _, x)| *x) {
                let new_decks: Vec<Deck> = top_card_info
                    .iter()
                    .map(|(deck, newlen, _)| {
                        let mut new_deck = (&**deck).clone();
                        new_deck.resize(*newlen);
                        new_deck
                    })
                    .collect();
                println!("Creating recursive game!");
                let mut subgame = Game::new(&new_decks);
                let subgame_result = subgame.play(true)?;

                for (deck, top, _) in top_card_info.drain(..) {
                    if deck.player == subgame_result.player {
                        deck.add_cards(top, &mut top_cards);
                    }
                }
            } else {
                // this is just a normal case
                let winning_card = top_card_info
                    .iter()
                    .map(|(_, x, _)| *x)
                    .max()
                    .ok_or(TopLevelError::UnknownError)?;
                println!("The winning card is {}", winning_card);
                let (winner, _, _) = top_card_info
                    .drain(..)
                    .find(|(_,x,_)| *x == winning_card)
                    .ok_or(TopLevelError::UnknownError)?;
                winner.add_cards(winning_card, &mut top_cards);
            }

            println!();
        }
    }
}

fn main() -> Result<(), TopLevelError> {
    let filename = env::args().nth(1).expect("No file argument given.");
    let contents = fs::read_to_string(filename)?;
    let mut lines = contents.lines();
    let deck1 = Deck::read(&mut lines)?;
    let deck2 = Deck::read(&mut lines)?;
    let decks = [deck1, deck2];
    let mut game1 = Game::new(&decks);

    let result = game1.play(true)?;
    let mut sum = 0;

    for (num, card) in result.cards.iter().rev().enumerate() {
        sum += (num + 1) * card;
    }

    println!("Winning deck: {}", result);
    println!("Final puzzle result: {}", sum);

    Ok(())
}
