use enum_map::{Enum, EnumMap};
use strum::EnumIter;
use crate::game_model::bidding::{Auction, Contract};
use crate::game_model::cards::{Deck, PlayerHand};
use crate::game_model::play::Play;

pub mod cards;
pub mod bidding;
pub mod play;

pub struct BridgeGame {
  board: Board,
  auction: Auction,
  play: Option<Play>,
  result: Option<HandResult>,
}

impl BridgeGame {
  pub fn new(board_num: u32) -> Self {
    let board = Board::new(board_num);
    let auction = Auction::new(dealer(board_num));
    BridgeGame {
      board,
      auction,
      play: None,
      result: None,
    }
  }

  pub fn board(&self) -> &Board {
    &self.board
  }

  pub fn player_hand(&self, seat: Seat) -> &PlayerHand {
    self.board().player_hand(seat)
  }
}

#[derive(Debug)]
pub struct Board {
  hands: EnumMap<Seat, PlayerHand>,
  number: u32,
}

impl Board {
  pub fn new(number: u32) -> Self {
    let deck = Deck::new();
    let hands = deck.deal_hands();
    Board { hands, number }
  }

  pub fn player_hand(&self, seat: Seat) -> &PlayerHand {
    &self.hands[seat]
  }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Enum, EnumIter)]
pub enum Seat {
  North,
  East,
  South,
  West,
}

impl Seat {
  pub fn partner(&self) -> Seat {
    use Seat::*;
    match self {
      North => South,
      East => West,
      South => North,
      West => East
    }
  }

  pub fn is_opponent(&self, other: Seat) -> bool {
    use Seat::*;
    match self {
      North => other == East || other == West,
      East => other == North || other == South,
      South => other == East || other == West,
      West => other == North || other == South,
    }
  }

  pub fn next_seat(&self) -> Self {
    use Seat::*;
    match self {
      North => East,
      East => South,
      South => West,
      West => North,
    }
  }


  pub fn prev_seat(&self) -> Self {
    use Seat::*;
    match self {
      North => West,
      East => North,
      South => East,
      West => South,
    }
  }
}

#[derive(Debug)]
pub enum HandResult {
  Passout,
  Played(Contract, i8),
}

impl HandResult {
  pub fn score(&self, vul: Vulnerability) -> i32 {
    match self {
      HandResult::Passout => 0,
      HandResult::Played(contract, diff) => contract.score(*diff, vul)
    }
  }
}

#[derive(Debug)]
pub enum Vulnerability {
  Neither,
  NS,
  EW,
  Both
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
