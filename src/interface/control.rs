use enum_map::EnumMap;
use crate::game_model::{Auction, Board, Call, Card, CompleteHand, Play, PlayerHand, Seat};
use crate::interface::view::BridgeView;

trait Player {
  fn receive_hand(&self, hand: &PlayerHand);
  fn get_call(&self, auction: &Auction) -> Call;
  fn get_play(&self, play: &Play) -> Card;
}

fn run_hand(board_num: u32,
            players: EnumMap<Seat, Box<dyn Player>>,
            view: BridgeView) -> CompleteHand {
  let mut board = Board::new(board_num);
  for seat in Seat::iter() {
    players[seat].receive_hand(hand.player_hand(seat));
  }

  let mut auction = board.auction();
  let auction_result = loop {
    let call = players[auction.current_bidder()].get_call(&auction);
    auction.make_call(call);
    if auction.is_complete() {
      break auction.complete();
    }
  }.unwrap();

  match auction_result {
    Ok(play) => run_play(play, players),
    Err(complete) => complete
  }
}

fn run_play(mut play: Play, players: EnumMap<Seat, Box<dyn Player>>) -> CompleteHand {
  let mut on_move = play.declarer().next_seat();
  let opening_lead = players[on_move].get_play();

  for _ in 1..52 {

  }
  play.complete().unwrap()
}
