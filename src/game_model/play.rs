use std::cmp::Ordering;
use std::collections::VecDeque;
use std::fmt::{Display, Formatter};
use std::ops::{Index, IndexMut};
use enum_map::{enum_map, EnumMap};
use strum::IntoEnumIterator;
use crate::game_model::{Board, HandResult, Seat};
use crate::game_model::bidding::{Bid, Call, Contract, DoubleLevel, Strain};
use crate::game_model::cards::{Card, PlayerHand, Suit};

#[derive(Debug)]
pub struct Play {
  contract: Contract,
  tricks: Vec<Trick>,
  declarer_tricks: u8,
  defense_tricks: u8,
}

impl Play {
  pub fn new(contract: Contract) -> Self {
    Play {
      contract,
      tricks: Vec::new(),
      declarer_tricks: 0,
      defense_tricks: 0,
    }
  }

  pub fn make_play(&mut self, seat: Seat, card: Card, hand: &PlayerHand) -> bool {
    // this play is a new trick if there is no incomplete trick in the trick vector
    if usize::from(self.declarer_tricks + self.defense_tricks) == self.tricks.len() {
      self.make_lead(seat, card)
    } else {
      self.follow(seat, card, hand)
    }
  }

  fn make_lead(&mut self, seat: Seat, card: Card) -> bool {
    let mut trick = EnumMap::default();
    trick[seat] = Some(card);
    self.tricks.push(Trick { cards: trick, leader: seat });
    true
  }

  fn follow(&mut self, seat: Seat, card: Card, hand: &PlayerHand) -> bool {
    let trick = self.tricks.last_mut().unwrap();
    if trick[seat].is_some() {
      return false;
    }
    let leader = trick.leader;
    let lead_suit = trick[leader].unwrap().suit();
    if card.suit() == lead_suit
        || !hand.has_any(lead_suit) {
      trick[seat] = Some(card);
    } else {
      return false;
    }
    if trick.cards.iter().all(|(_, v)| v.is_some()) {
      let winner = trick.winner(self.contract.strain()).unwrap();
      if winner.is_opponent(self.declarer()) {
        self.defense_tricks += 1;
      } else {
        self.declarer_tricks += 1;
      }
    }
    true
  }

  pub fn declarer(&self) -> Seat {
    self.contract.declarer()
  }

  pub fn tricks(&self) -> &Vec<Trick> {
    &self.tricks
  }

  pub fn is_complete(&self) -> bool {
    self.declarer_tricks + self.defense_tricks == 13
  }

  pub fn result(&self) -> Option<HandResult> {
    if self.is_complete() {
      let target = self.contract.level() + 6;
      let made = self.declarer_tricks;
      let result = HandResult::Played(self.contract, made as i8 - target as i8);
      Some(result)
    } else {
      None
    }
  }
}

#[derive(Debug)]
pub struct Trick {
  cards: EnumMap<Seat, Option<Card>>,
  leader: Seat,
}

impl Trick {
  pub fn winner(&self, trump: Strain) -> Option<Seat> {
    if self.cards.iter().any(|(_, v)| v.is_none()) {
      None
    } else {
      let lead = self.cards[self.leader].unwrap();
      let mut best = lead;
      let mut winner = self.leader;
      let mut seat = self.leader.next_seat();
      while seat != self.leader {
        let card = self.cards[seat].unwrap();
        match best.compare_with_trump(card, trump) {
          Ordering::Less => {
            best = card;
            winner = seat;
          },
          _ => {}
        }
        seat = seat.next_seat()
      }
      Some(winner)
    }
  }
}

impl Index<Seat> for Trick {
  type Output = Option<Card>;

  fn index(&self, index: Seat) -> &Self::Output {
    &self.cards[index]
  }
}

impl IndexMut<Seat> for Trick {
  fn index_mut(&mut self, index: Seat) -> &mut Self::Output {
    &mut self.cards[index]
  }
}
