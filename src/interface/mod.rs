use cursive::{Cursive, CursiveRunnable};
use cursive::views::{DummyView, LinearLayout};
use crate::game_model::bidding::{Auction, Call};
use crate::game_model::{Board, BridgeGame, dealer, HandResult, Seat};
use crate::game_model::cards::{Card, PlayerHand};
use crate::game_model::play::Play;

mod view;
mod control;

trait Player {
  fn receive_hand(&self, hand: &PlayerHand);
  fn get_call(&self, auction: &Auction) -> Call;
  fn get_play(&self, play: &Play) -> Card;
  fn notify_dummy(&self, dummy: &PlayerHand);
}

pub fn run() {
  let mut siv = cursive::default();
  siv.add_global_callback('q', Cursive::quit);

  let game = BridgeGame::new(1);
  initialize_layout(&mut siv, &game);
}

fn initialize_layout(siv: &mut Cursive, game: &BridgeGame) {
  let mut columns = LinearLayout::horizontal();
  columns.add_child(DummyView);
  // columns.add_child();
}
