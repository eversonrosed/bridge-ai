use std::cmp::Ordering;
use std::collections::VecDeque;
use std::fmt::{Display, Formatter};
use enum_map::{Enum, EnumMap};
use crate::game_model::{Board, CompleteHand, dealer, Seat};
use crate::game_model::cards::Suit;
use crate::game_model::play::Play;

#[derive(Debug)]
pub struct Auction {
  board: Board,
  calls: VecDeque<Call>,
  dealer: Seat,
  declarers: EnumMap<Strain, Option<Seat>>,
  current_bidder: Seat,
  highest_bid: Option<(Bid, Seat)>,
  doubled: DoubleLevel,
  passes: u8,
}

impl Auction {
  /** Creates a new empty auction.
   */
  pub fn new(board: Board) -> Auction {
    let dealer = dealer(board.number);
    Auction {
      board,
      calls: VecDeque::new(),
      dealer,
      declarers: EnumMap::default(),
      current_bidder: dealer,
      highest_bid: None,
      doubled: DoubleLevel::Undoubled,
      passes: 0,
    }
  }

  /** Attempts to add a call to the auction. If the call is legal, it is added to the call list,
        the other data is updated, and the function returns `true`. If the call is illegal, no data
        is modified and the function returns `false`.
   */
  pub fn make_call(&mut self, call: Call) -> bool {
    let success = match call {
      Call::Bid(bid) => if let Some((high_bid, _)) = self.highest_bid {
        if bid > high_bid {
          self.highest_bid = Some((bid, self.current_bidder));
          self.doubled = DoubleLevel::Undoubled;
          self.passes = 0;
          if self.declarers[bid.strain].is_none() {
            self.declarers[bid.strain] = Some(self.current_bidder);
          }
          true
        } else { // underbid
          false
        }
      } else { // no bid yet made
        self.highest_bid = Some((bid, self.current_bidder));
        self.passes = 0;
        true
      }
      Call::Pass => {
        self.passes += 1;
        true
      }
      Call::Double => if let Some((_, bid_seat)) = self.highest_bid {
        if self.current_bidder.is_opponent(bid_seat)
            && self.doubled == DoubleLevel::Undoubled {
          self.doubled = DoubleLevel::Doubled;
          self.passes = 0;
          true
        } else {
          false // can only double opponents' undoubled contract
        }
      } else {
        false // can't double if there's no contract
      }
      Call::Redouble => if let Some((_, bid_seat)) = self.highest_bid {
        if !self.current_bidder.is_opponent(bid_seat)
            && self.doubled == DoubleLevel::Doubled {
          self.doubled = DoubleLevel::Redoubled;
          true
        } else {
          false // can only redouble if your side's contract is doubled
        }
      } else {
        false // can't redouble if there's no contract
      }
    };
    if success {
      self.calls.push_back(call);
      self.current_bidder = self.current_bidder.next_seat();
    }
    success
  }

  pub fn current_bidder(&self) -> Seat {
    self.current_bidder
  }

  pub fn is_complete(&self) -> bool {
    if let Some(_) = self.highest_bid {
      self.passes == 3 // three passes end the auction if a bid has been made
    } else {
      self.passes == 4 // four passes end the auction if no bid has been made
    }
  }

  pub fn complete(self) -> Result<Result<Play, CompleteHand>, Self> {
    if self.is_complete() {
      if let Some((bid, _)) = self.highest_bid {
        let declarer: Seat = self.declarers[bid.strain].unwrap(); // a declarer is always set
        let play = Play::from_auction(
          self.board,
          self.calls,
          self.dealer,
          bid,
          self.doubled,
          declarer
        );
        Ok(Ok(play))
      } else {
        Ok(Err(CompleteHand::Passout(self.board)))
      }
    } else {
      Err(self)
    }
  }

  pub fn board(&self) -> &Board {
    &self.board
  }
}

impl Display for Auction {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let complete_str = if self.is_complete() {
      "Complete"
    } else {
      "In Progress"
    };
    writeln!(f, "Auction Status: {}", complete_str)?;
    writeln!(f, "North   East    South   West")?;
    let mut current_seat = Seat::North;
    while current_seat != self.dealer { // align first call
      write!(f, "        ");
      current_seat = current_seat.next_seat();
    }
    for call in self.calls.iter() {
      let call_str = match call {
        Call::Bid(bid) => bid.to_string(),
        Call::Pass => "Pass".to_string(),
        Call::Double => "X".to_string(),
        Call::Redouble => "XX".to_string(),
      };
      if current_seat == Seat::West {
        writeln!(f, "{}", call_str)?;
      } else {
        write!(f, "{:8}", call_str)?;
      }
      current_seat = current_seat.next_seat();
    }
    if !self.is_complete() {
      write!(f, "...")?;
    }
    Ok(())
  }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Enum)]
pub enum Strain {
  Trump(Suit),
  Notrump,
}

impl PartialOrd for Strain {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for Strain {
  fn cmp(&self, other: &Self) -> Ordering {
    if self == other {
      Ordering::Equal
    } else {
      if let Strain::Trump(trump) = self {
        if let Strain::Trump(other_trump) = other { // compare the suits
          trump.cmp(&other_trump)
        } else { // other is notrump, so it's higher
          Ordering::Less
        }
      } else { // this is notrump, so it's higher
        Ordering::Greater
      }
    }
  }
}

impl Display for Strain {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let trump_str = if let Strain::Trump(trump) = self {
      trump.to_string()
    } else {
      "NT".to_string()
    };
    f.write_str(&trump_str)?;
    Ok(())
  }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Bid {
  level: u8,
  strain: Strain,
}

impl Display for Bid {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}{}", self.level, self.strain)?;
    Ok(())
  }
}

#[derive(Debug, Copy, Clone)]
pub enum Call {
  Bid(Bid),
  Pass,
  Double,
  Redouble,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum DoubleLevel {
  Undoubled,
  Doubled,
  Redoubled,
}

#[derive(Debug)]
pub struct Contract(Bid, DoubleLevel, Seat);

impl Contract {
  pub fn new(bid: Bid, doubled: DoubleLevel, declarer: Seat) -> Self {
    Contract { 0: bid, 1: doubled, 2: declarer }
  }

  pub fn level(&self) -> u8 {
    self.0.level
  }

  pub fn strain(&self) -> Strain {
    self.0.strain
  }

  pub fn doubled(&self) -> DoubleLevel {
    self.1
  }

  pub fn declarer(&self) -> Seat {
    self.2
  }
}
