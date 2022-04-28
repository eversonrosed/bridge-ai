use std::cmp::Ordering;
use std::collections::VecDeque;
use std::fmt::{Display, Formatter};
use enum_map::{Enum, EnumMap};
use crate::game_model::{Board, dealer, HandResult, Seat, Vulnerability};
use crate::game_model::cards::Suit;
use crate::game_model::play::Play;

#[derive(Debug)]
pub struct Auction {
  calls: VecDeque<Call>,
  dealer: Seat,
  ns_declarers: EnumMap<Strain, Option<Seat>>,
  ew_declarers: EnumMap<Strain, Option<Seat>>,
  current_bidder: Seat,
  highest_bid: Option<(Bid, Seat)>,
  doubled: DoubleLevel,
  passes: u8,
}

impl Auction {
  /** Creates a new empty auction.
   */
  pub fn new(dealer: Seat) -> Auction {
    Auction {
      calls: VecDeque::new(),
      dealer,
      ns_declarers: EnumMap::default(),
      ew_declarers: EnumMap::default(),
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
          self.set_declarer(bid.strain, self.current_bidder);
          true
        } else { // underbid
          false
        }
      } else { // no bid yet made
        self.highest_bid = Some((bid, self.current_bidder));
        self.set_declarer(bid.strain, self.current_bidder);
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

  fn set_declarer(&mut self, strain: Strain, seat: Seat) {
    let declarers = match seat {
      Seat::North | Seat::South => &mut self.ns_declarers,
      Seat::East | Seat::West => &mut self.ew_declarers,
    };
    if declarers[strain].is_none() {
      declarers[strain] = Some(seat);
    }
  }

  pub fn current_bidder(&self) -> Seat {
    self.current_bidder
  }

  pub fn len(&self) -> usize {
    self.calls.len()
  }

  pub fn is_complete(&self) -> bool {
    if let Some(_) = self.highest_bid {
      self.passes == 3 // three passes end the auction if a bid has been made
    } else {
      self.passes == 4 // four passes end the auction if no bid has been made
    }
  }

  pub fn play(&self) -> Option<Play> {
    if self.is_complete() {
      if let Some((bid, seat)) = self.highest_bid {
        let declarer: Seat = match seat {
          Seat::North | Seat::South => &self.ns_declarers,
          Seat::East | Seat::West => &self.ew_declarers,
        }[bid.strain].unwrap(); // a declarer is always set
        let play = Play::new(Contract::new(bid, self.doubled, declarer));
        Some(play)
      } else {
        None
      }
    } else {
      None
    }
  }
}

impl Display for Auction {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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

impl Strain {
  fn trick_score(&self, tricks: i32) -> i32 {
    let per_trick = self.score_per_trick();
    match self {
      Strain::Notrump => per_trick * tricks + 10,
      _ => per_trick * tricks
    }
  }

  fn score_per_trick(&self) -> i32 {
    match self {
      Strain::Trump(suit) => match suit {
        Suit::Clubs | Suit::Diamonds => 20,
        Suit::Hearts | Suit::Spades => 30,
      },
      Strain::Notrump => 30,
    }
  }
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

impl Bid {
  pub fn from(level: u8, strain: Strain) -> Self {
    Bid { level, strain }
  }
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

impl DoubleLevel {
  pub fn score_for_set(&self, set_by: i32, vul: bool) -> i32 {
    if set_by <= 0 {
      return 0;
    }
    use DoubleLevel::*;
    match (self, vul) {
      (Undoubled, false) => -50 * set_by,
      (Undoubled, true) => -100 * set_by,
      (Doubled, false) => match set_by {
        1 => -100,
        2 => -300,
        _ => -300 * set_by + 400,
      },
      (Doubled, true) => -300 * set_by + 100,
      (Redoubled, false) => match set_by {
        1 => -200,
        2 => -600,
        _ => -600 * set_by + 800,
      }
      (Redoubled, true) => -600 * set_by + 200
    }
  }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
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

  pub fn score(&self, diff: i8, vulnerable: Vulnerability) -> i32 {
    let vul = match vulnerable {
      Vulnerability::Neither => false,
      Vulnerability::NS => self.declarer().is_opponent(Seat::East),
      Vulnerability::EW => self.declarer().is_opponent(Seat::North),
      Vulnerability::Both => true,
    };
    if diff < 0 {
      self.doubled().score_for_set(-diff as i32, vul)
    } else {
      let trick_score = self.trick_score(self.level() as i32 + diff as i32);
      let game_bonus = if vul { 500 } else { 300 };
      let slam_bonus = if vul { 750 } else { 500 };
      let grand_bonus = if vul { 1500 } else { 1000 };
      let double_bonus = match self.doubled() {
        DoubleLevel::Undoubled => 0,
        DoubleLevel::Doubled => 50,
        DoubleLevel::Redoubled => 100,
      };
      let bonus = match self.level() {
        7 => grand_bonus + game_bonus,
        6 => slam_bonus + game_bonus,
        5 => game_bonus,
        4 => match self.strain() {
          Strain::Trump(Suit::Clubs) | Strain::Trump(Suit::Diamonds) => 50,
          _ => game_bonus,
        },
        3 => match self.strain() {
          Strain::Notrump => game_bonus,
          _ => 50
        },
        _ => trick_score + 50,
      };
      bonus + trick_score + double_bonus
    }
  }

  fn trick_score(&self, tricks: i32) -> i32 {
    let base = self.strain().trick_score(tricks);
    let multiplier = match self.doubled() {
      DoubleLevel::Undoubled => 1,
      DoubleLevel::Doubled => 2,
      DoubleLevel::Redoubled => 4,
    };
    base * multiplier
  }
}
