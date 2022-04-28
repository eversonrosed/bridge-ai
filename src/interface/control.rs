// use enum_map::EnumMap;
// use strum::IntoEnumIterator;
// use crate::game_model::{
//   Board,
//   Seat,
//   HandResult,
//   bidding::{Auction, Call},
//   cards::{Card, PlayerHand},
//   play::Play,
// };
// use crate::interface::Player;

// fn run_hand(board_num: u32,
//             players: EnumMap<Seat, Box<dyn Player>>,
//             view: BridgeView) -> CompleteHand {
//   let mut board = Board::new(board_num);
//   for seat in Seat::iter() {
//     players[seat].receive_hand(board.player_hand(seat));
//   }
//
//   let mut auction = board.auction();
//   let auction_result = loop {
//     let call = players[auction.current_bidder()].get_call(&auction);
//     auction.make_call(call);
//     if auction.is_complete() {
//       break auction.complete();
//     }
//   }.unwrap();
//
//   match auction_result {
//     Ok(play) => run_play(play, players, view),
//     Err(complete) => complete
//   }
// }
//
// fn run_play(mut play: Play, players: EnumMap<Seat, Box<dyn Player>>, view: BridgeView) -> CompleteHand {
//
//   let mut on_move = play.declarer().next_seat();
//   let mut card_played = players[on_move].get_play(&play);
//   play.make_play(on_move, card_played);
//   let dummy_seat = on_move.next_seat();
//   for (seat, player) in players.iter() {
//     if seat == dummy_seat {
//       continue
//     } else {
//       player.notify_dummy(play.dummy_hand())
//     }
//   }
//
//   for _ in 1..52 {
//
//   }
//   play.complete().unwrap()
// }
