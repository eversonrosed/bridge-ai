use std::cmp::Ordering;
use std::collections::VecDeque;
use std::fmt::{Display, Formatter};
use std::ops::{Index, IndexMut};
use enum_map::{enum_map, EnumMap};
use strum::IntoEnumIterator;
use crate::game_model::{Board, BridgeHand, CompleteHand, Seat};
use crate::game_model::bidding::{Bid, Call, Contract, DoubleLevel, Strain};
use crate::game_model::cards::{Card, Suit};

#[derive(Debug)]
pub struct Play {
  board: Board,
  auction: VecDeque<Call>,
  dealer: Seat,
  contract: Contract,
  tricks: Vec<Trick>,
  declarer_tricks: u8,
  defense_tricks: u8,
}

impl Play {
  pub fn from_auction(
    board: Board,
    calls: VecDeque<Call>,
    dealer: Seat,
    bid: Bid,
    doubled: DoubleLevel,
    declarer: Seat,
  ) -> Self {
    Play {
      board,
      auction: calls,
      dealer,
      contract: Contract::new(bid, doubled, declarer),
      tricks: Vec::new(),
      declarer_tricks: 0,
      defense_tricks: 0,
    }
  }

  pub fn make_play(&mut self, seat: Seat, card: Card) {
    // this play is a new trick if there is no incomplete trick in the trick vector
    if usize::from(self.declarer_tricks + self.defense_tricks) == self.tricks.len() {
      self.make_lead(seat, card);
    } else {
      self.follow(seat, card);
    }
  }

  fn make_lead(&mut self, seat: Seat, card: Card) {
    let mut trick = EnumMap::default();
    trick[seat] = Some(card);
    self.tricks.push(Trick { cards: trick, leader: seat });
  }

  fn follow(&mut self, seat: Seat, card: Card) {
    let trick = self.tricks.last_mut().unwrap();
    if trick[seat].is_some() {
      return;
    }
    let leader = trick.leader;
    let lead_suit = trick[leader].unwrap().suit();
    if card.suit() == lead_suit
        || !self.board.hands[seat].has_any(lead_suit) {
      trick[seat] = Some(card);
    }
    if trick.cards.iter().all(|(_, v)| v.is_some()) {
      let winner = trick.winner(self.contract.strain()).unwrap();
      if winner.is_opponent(self.declarer()) {
        self.defense_tricks += 1;
      } else {
        self.declarer_tricks += 1;
      }
    }
  }

  pub fn declarer(&self) -> Seat {
    self.contract.declarer()
  }

  pub fn is_complete(&self) -> bool {
    self.declarer_tricks + self.defense_tricks == 13
  }

  pub fn complete(self) -> BridgeHand {
    let target = self.contract.level() + 6;
    let made = self.declarer_tricks;
    let result = if made >= target {
      HandResult::Made(made - target)
    } else {
      HandResult::Set(target - made)
    };
    BridgeHand::Complete(CompleteHand::Played(PlayedHand {
      board: self.board,
      auction: self.auction,
      dealer: self.dealer,
      contract: Some(self.contract),
      tricks: self.tricks,
      result,
    }))
  }

  pub fn board(&self) -> &Board {
    &self.board
  }
}

impl Display for Play {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let dummy = self.contract.declarer().partner();
    let dummy_hand = self.board.hands[dummy].clone();
    let dummy_sorted = dummy_hand.sort();
    let dummy_text: EnumMap<Suit, String> = dummy_sorted.map(|suit, cards| {
      cards.iter().fold(suit.to_string() + " ", |s, card| s + &card.rank().to_string())
    });
    let last_trick: EnumMap<Seat, String> = self.tricks.last()
        .map_or(EnumMap::default(), |trick| {
          trick.cards.map(|_, opt_card| opt_card.map_or(String::new(), |card| card.to_string()))
        });
    match dummy {
      Seat::North => {
        for suit in Suit::iter().rev() {
          writeln!(f, "{:5}{:15}", "", dummy_text[suit])?;
        }
        writeln!(f, "{:9}NORTH", "")?;
        writeln!(f, "{:11}{}", "", last_trick[Seat::North])?;
        writeln!(f, "WEST {:2}     {:2} EAST", last_trick[Seat::West], last_trick[Seat::East])?;
        writeln!(f, "{:11}{}", "", last_trick[Seat::South])?;
        writeln!(f, "{:9}SOUTH", "")?;
      }
      Seat::East => {}
      Seat::South => {
        writeln!(f, "{:9}NORTH", "")?;
        writeln!(f, "{:11}{}", "", last_trick[Seat::North])?;
        writeln!(f, "WEST {:2}     {:2} EAST", last_trick[Seat::West], last_trick[Seat::East])?;
        writeln!(f, "{:11}{}", "", last_trick[Seat::South])?;
        writeln!(f, "{:9}SOUTH", "")?;
        for suit in Suit::iter().rev() {
          writeln!(f, "{:5}{:15}", "", dummy_text[suit])?;
        }
      }
      Seat::West => {}
    }
    Ok(())
  }
}

#[derive(Debug)]
struct Trick {
  cards: EnumMap<Seat, Option<Card>>,
  leader: Seat,
}

impl Trick {
  fn winner(&self, trump: Strain) -> Option<Seat> {
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

#[derive(Debug)]
pub struct PlayedHand {
  board: Board,
  auction: VecDeque<Call>,
  dealer: Seat,
  contract: Option<Contract>,
  tricks: Vec<Trick>,
  result: HandResult,
}

impl PlayedHand {
  pub fn board(&self) -> &Board {
    &self.board
  }
}

impl Display for PlayedHand {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    todo!()
  }
}

#[derive(Debug)]
pub enum HandResult {
  Made(u8),
  Set(u8),
}
