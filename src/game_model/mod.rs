use enum_map::{Enum, EnumMap};
use crate::game_model::bidding::Auction;
use crate::game_model::cards::{Deck, PlayerHand};
use crate::game_model::play::{Play, PlayedHand};

mod cards;
mod bidding;
mod play;

pub enum BridgeHand {
  Dealt(Board),
  Bidding(Auction),
  Play(Play),
  Complete(CompleteHand),
}

impl BridgeHand {
  pub fn new(board_num: u32) -> BridgeHand {
    let deck = Deck::new();
    let hands = deck.deal_hands();
    let board = Board { hands, number: board_num };
    BridgeHand::Dealt(board)
  }

  pub fn start_bidding(self) -> Self {
    if let BridgeHand::Dealt(board) = self {
      let auction = Auction::new(board);
      BridgeHand::Bidding(auction)
    } else {
      self
    }
  }

  pub fn board(&self) -> &Board {
    match self {
      BridgeHand::Dealt(board) => &board,
      BridgeHand::Bidding(auction) => auction.board(),
      BridgeHand::Play(play) => play.board(),
      BridgeHand::Complete(complete) => match complete {
        CompleteHand::Played(played) => played.board(),
        CompleteHand::Passout(board) => &board,
      },
    }
  }

  fn player_hand(&self, seat: Seat) -> &PlayerHand {
    &self.board().hands[seat]
  }
}

#[derive(Debug)]
pub struct Board {
  hands: EnumMap<Seat, PlayerHand>,
  number: u32,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Enum)]
pub enum Seat {
  North,
  East,
  South,
  West,
}

impl Seat {
  fn partner(&self) -> Seat {
    use Seat::*;
    match self {
      North => South,
      East => West,
      South => North,
      West => East
    }
  }

  fn is_opponent(&self, other: &Seat) -> bool {
    use Seat::*;
    match self {
      North => *other == East || *other == West,
      East => *other == North || *other == South,
      South => *other == East || *other == West,
      West => *other == North || *other == South,
    }
  }

  fn next_seat(&self) -> Self {
    use Seat::*;
    match self {
      North => East,
      East => South,
      South => West,
      West => North,
    }
  }
}

pub enum CompleteHand {
  Played(PlayedHand),
  Passout(Board),
}

pub fn dealer(board_num: u32) -> Seat {
  match board_num % 4 {
    0 => Seat::West,
    1 => Seat::North,
    2 => Seat::East,
    3 => Seat::South,
    _ => unreachable!()
  }
}
