use std::os::raw::c_int;
use enum_map::EnumMap;
use crate::ai::dds::dds_deal;
use crate::ai::dds_bindings::{AnalysePlayBin, boards, deal, futureTricks, SolveAllBoards, SolveAllBoardsBin, solvedBoards};
use crate::game_model::bidding::{Auction, Call};
use crate::game_model::cards::{Card, PlayerHand, Rank, Suit};
use crate::game_model::{Board, HandResult, Seat};
use crate::game_model::play::Play;
use crate::interface::Player;

pub struct AlphaMuPlayer {
  hand: PlayerHand,
  dummy: Option<PlayerHand>,
}

impl AlphaMuPlayer {
  fn alpha_mu_search(&self, state: &Play, moves: u32, worlds: &mut Vec<World>) -> Card {
    if self.stop(state, moves, worlds) {
      // sort it out
    }
    Card::from(Suit::Spades, Rank::Ace)
  }

  fn stop(&self, state: &Play, moves: u32, worlds: &mut Vec<World>) -> bool {
    if state.is_complete() {
      let result = state.result().unwrap();
      match result {
        HandResult::Played(contract, diff) => {
          if diff >= 0 {
            for w in worlds {
              w.result = Some(true);
            }
          } else {
            for w in worlds {
              w.result = Some(false);
            }
          }
        }
        HandResult::Passout => unreachable!()
      }
      true
    } else if moves == 0 {
      Self::double_dummy_solve(worlds);
      true
    } else {
      false
    }
  }

  fn double_dummy_solve(worlds: &mut Vec<World>) {
    let num_boards = worlds.len() as c_int;
    let mut deals = [deal {
      trump: 0,
      first: 0,
      currentTrickSuit: [0; 3],
      currentTrickRank: [0; 3],
      remainCards: [[0; 4]; 4],
    }; 200];
    let mut target = [0; 200];
    let solutions = [1; 200];
    let mode = [1; 200];
    worlds
        .iter()
        .enumerate()
        .for_each(|(i, w)| {
          deals[i] = dds_deal(&w.play, &w.board);
          target[i] = (w.play.contract().level() + 6 - w.play.declarer_tricks()) as c_int;
        });
    let mut boards = boards {
      noOfBoards: num_boards,
      deals,
      target,
      solutions,
      mode
    };
    let fut = futureTricks {
      nodes: 0,
      cards: 0,
      suit: [0; 13],
      rank: [0; 13],
      equals: [0; 13],
      score: [0; 13],
    };
    let mut solved = solvedBoards {
      noOfBoards: num_boards,
      solvedBoard: [fut; 200]
    };
    unsafe {
      SolveAllBoardsBin(&mut boards, &mut solved);
    }
  }
}

impl Player for AlphaMuPlayer {
  fn new(hand: &PlayerHand) -> Self {
    AlphaMuPlayer { hand: hand.clone(), dummy: None }
  }

  fn get_call(&self, auction: &Auction) -> Call {
    Call::Pass
  }

  fn get_play(&mut self, play: &Play, board: &Board) -> Card {
    let mut worlds = vec![World::new(play.clone(), board.clone())];
    self.alpha_mu_search(play, 0, &mut worlds)
  }

  fn notify_dummy(&mut self, dummy: &PlayerHand) {
    self.dummy = Some(dummy.clone())
  }
}

pub struct World {
  play: Play,
  board: Board,
  result: Option<bool>,
  valid: bool,
}

impl World {
  fn new(play: Play, board: Board) -> Self {
    World { play, board, result: None, valid: true }
  }
}
