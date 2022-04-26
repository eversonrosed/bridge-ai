use std::collections::VecDeque;
use std::fmt::{Display, Formatter};
use enum_map::EnumMap;
use strum::IntoEnumIterator;
use crate::game_model::{Board, BridgeHand, CompleteHand, Seat};
use crate::game_model::bidding::{Bid, Call, Contract, DoubleLevel};
use crate::game_model::cards::{Card, Suit};

type Trick = EnumMap<Seat, Option<Card>>;

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

  pub fn declarer(&self) -> Seat {
    self.contract.declarer()
  }

  pub fn finish(self) -> BridgeHand {
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
          trick.map(|_, opt_card| opt_card.map_or(String::new(), |card| card.to_string()))
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
