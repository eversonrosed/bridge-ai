use strum::IntoEnumIterator;
use std::os::raw::{c_int, c_uint};
use crate::ai::dds_bindings::deal;
use crate::game_model::cards::{PlayerHand, Rank, Suit};
use crate::game_model::{Board, Seat, Vulnerability};
use crate::game_model::bidding::Strain;
use crate::game_model::play::Play;

pub fn dds_hand(seat: Seat) -> c_int {
  match seat {
    Seat::North => 0,
    Seat::East => 1,
    Seat::South => 2,
    Seat::West => 3,
  }
}

pub fn dds_suit(suit: Suit) -> c_int {
  match suit {
    Suit::Clubs => 0,
    Suit::Diamonds => 1,
    Suit::Hearts => 2,
    Suit::Spades => 3,
  }
}

pub fn dds_strain(strain: Strain) -> c_int {
  match strain {
    Strain::Trump(suit) => dds_suit(suit),
    Strain::Notrump => 4,
  }
}

pub fn dds_rank(rank: Rank) -> c_int {
  match rank {
    Rank::Two => 2,
    Rank::Three => 3,
    Rank::Four => 4,
    Rank::Five => 5,
    Rank::Six => 6,
    Rank::Seven => 7,
    Rank::Eight => 8,
    Rank::Nine => 9,
    Rank::Ten => 10,
    Rank::Jack => 11,
    Rank::Queen => 12,
    Rank::King => 13,
    Rank::Ace => 14,
  }
}

pub fn dds_vul(vul: Vulnerability) -> c_int {
  match vul {
    Vulnerability::Neither => 0,
    Vulnerability::NS => 2,
    Vulnerability::EW => 3,
    Vulnerability::Both => 1,
  }
}

pub fn dds_remain_cards(board: &Board) -> [[c_uint; 4usize]; 4usize] {
  let mut result = [[0u32; 4]; 4];
  for seat in Seat::iter() {
    let sorted = board.player_hand(seat).sort();
    let mut result_hand = &mut result[dds_hand(seat) as usize];
    for (suit, &ranks) in sorted.iter() {
      let mut result_suit = &mut result_hand[dds_suit(suit) as usize];
      for rank in ranks {
        *result_suit |= 1 << dds_rank(rank);
      }
    }
  }
  result
}

pub fn dds_deal(play: &Play, board: &Board) -> deal {
  let strain = play.contract().strain();
  let leader = {
    let opening_leader = play.declarer().next_seat();
    play.tricks().last().map_or(opening_leader, |t| {
      t.winner(strain).unwrap_or(t.leader())
    })
  };
  let (current_suit, current_rank) = play.tricks().last().map_or(([0; 3], [0; 3]), |t| {
    let mut l = leader;
    let mut suits = [0; 3];
    let mut ranks = [0; 3];
    while l.next_seat() != leader {
      if let Some(card) = t[l] {
        let idx = dds_hand(l) as usize;
        suits[idx] = dds_suit(card.suit());
        ranks[idx] = dds_rank(card.rank());
      }
    }
    (suits, ranks)
  });
  deal {
    trump: dds_strain(strain),
    first: dds_hand(leader),
    currentTrickSuit: current_suit,
    currentTrickRank: current_rank,
    remainCards: dds_remain_cards(board),
  }
}
