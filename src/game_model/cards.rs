use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use enum_map::{Enum, EnumMap};
use itertools::Product;
use rand::prelude::*;
use strum::EnumIter;
use strum::IntoEnumIterator;
use crate::game_model::Seat;

#[derive(Debug)]
pub struct Deck {
  cards: Vec<Card>,
}

impl Deck {
  /** Creates a new shuffled 52-card deck.
   */
  pub fn new() -> Deck {
    Product::new(Suit::iter(), Rank::iter()).map(|(suit, rank)| Card { suit, rank }).collect()
  }

  /** Deals a deck into four hands. This operation consumes the deck.
   */
  pub fn deal_hands(mut self) -> EnumMap<Seat, PlayerHand> {
    let mut hands: EnumMap<_, PlayerHand> = EnumMap::default();
    let mut current_seat = Seat::North;
    for card in self.cards.drain(..) {
      hands[current_seat].cards.push(card);
      current_seat = current_seat.next_seat();
    }
    hands
  }
}

impl FromIterator<Card> for Deck {
  /** Collects an iterator of cards into an array, shuffles the array, then converts the array to a
                 deck. Panics if the iterator does not contain exactly 52 cards.
   */
  fn from_iter<T: IntoIterator<Item=Card>>(iter: T) -> Self {
    let mut cards: Vec<Card> = iter.into_iter().collect();
    let mut rng = thread_rng();
    cards.shuffle(&mut rng);
    Deck { cards }
  }
}

#[derive(Debug, Clone, Default)]
pub struct PlayerHand {
  cards: Vec<Card>,
}

impl PlayerHand {
  pub fn sort(&self) -> EnumMap<Suit, Vec<Card>> {
    let mut sorted_hand: EnumMap<_, Vec<Card>> = EnumMap::default();
    for card in &self.cards {
      sorted_hand[card.suit].push(*card);
    }
    for suit in Suit::iter() {
      sorted_hand[suit].sort_unstable_by(|x, y| y.partial_cmp(x).unwrap()); // ok because same suit
    }
    sorted_hand
  }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Card {
  suit: Suit,
  rank: Rank,
}

impl Card {
  /** Compares two cards with the specified trump suit. If there is no comparison, the first card
                wins. (This occurs when the two cards have different suits and neither is trump, so the
                leader holds. It is assumed that `self` is always either the led suit or trump.)
   */
  pub fn compare_with_trump(&self, other: &Card, trump: &Option<Suit>) -> Ordering {
    if let Some(cmp) = self.partial_cmp(other) { // the cards have the same suit, so the usual order applies
      cmp
    } else { // the cards are different suits, so only one can be a trump
      if let Some(trump_suit) = trump { // only look at trump if there is a trump suit
        if self.suit == *trump_suit { // self is trump, so it wins
          Ordering::Greater
        } else if other.suit == *trump_suit { // other is trump, so it wins
          Ordering::Less
        } else { // neither is trump, so first card wins
          Ordering::Greater
        }
      } else { // at notrump, a card of a different suit always loses
        Ordering::Greater
      }
    }
  }

  pub fn suit(&self) -> Suit {
    self.suit
  }

  pub fn rank(&self) -> Rank {
    self.rank
  }
}

impl PartialOrd for Card {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    let rank_order = self.rank.cmp(&other.rank);
    if rank_order == Ordering::Equal {
      if self == other {
        Some(Ordering::Equal)
      } else {
        None
      }
    } else {
      Some(rank_order)
    }
  }
}

impl Display for Card {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}{}", self.suit, self.rank)?;
    Ok(())
  }
}

#[derive(Debug, Copy, Clone, EnumIter, Eq, PartialEq, Ord, PartialOrd, Enum)]
pub enum Suit {
  Clubs,
  Diamonds,
  Hearts,
  Spades,
}

impl Display for Suit {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let suit_str = match self {
      Suit::Clubs => "♣",
      Suit::Diamonds => "♦",
      Suit::Hearts => "♥",
      Suit::Spades => "♠",
    };
    f.write_str(suit_str)?;
    Ok(())
  }
}

#[derive(Debug, Copy, Clone, EnumIter, Ord, PartialOrd, Eq, PartialEq)]
pub enum Rank {
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
  Ace,
}

impl Display for Rank {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let rank_str = match self {
      Rank::Two => "2",
      Rank::Three => "3",
      Rank::Four => "4",
      Rank::Five => "5",
      Rank::Six => "6",
      Rank::Seven => "7",
      Rank::Eight => "8",
      Rank::Nine => "9",
      Rank::Ten => "T",
      Rank::Jack => "J",
      Rank::Queen => "Q",
      Rank::King => "K",
      Rank::Ace => "A",
    };
    f.write_str(rank_str)?;
    Ok(())
  }
}
