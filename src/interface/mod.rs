// use cursive::Cursive;
// use cursive::views::{DummyView, LinearLayout};
use crate::game_model::bidding::{Auction, Call};
use crate::game_model::Board;
// use crate::game_model::BridgeGame;
use crate::game_model::cards::{Card, PlayerHand};
use crate::game_model::play::Play;
//
// mod view;
// mod control;
// mod messages;
//
pub trait Player {
  fn new(hand: &PlayerHand) -> Self;
  fn get_call(&self, auction: &Auction) -> Call;
  fn get_play(&mut self, play: &Play, board: &Board) -> Card;
  fn notify_dummy(&mut self, dummy: &PlayerHand);
}
//
// pub fn run() {
//   let mut siv = cursive::default();
//   siv.add_global_callback('q', Cursive::quit);
//
//   let game = BridgeGame::new(1);
//   initialize_layout(&mut siv, &game);
// }
//
// fn initialize_layout(siv: &mut Cursive, game: &BridgeGame) {
//   let mut columns = LinearLayout::horizontal();
//   columns.add_child(DummyView);
//   // columns.add_child();
// }
