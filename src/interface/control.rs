use enum_map::EnumMap;

// use crate::game_model::{Auction, BridgeHand, Call, Card, Play, PlayerHand, Seat};
// use crate::game_model::view::BridgeView;

// trait Player {
//   fn receive_hand(&self, hand: &PlayerHand);
//   fn get_call(&self, auction: &Auction) -> Call;
//   fn get_play(&self, play: &Play) -> Card;
// }

// fn run_hand(board_num: u32,
//             players: EnumMap<Seat, Box<dyn Player>>,
//             view: BridgeView) -> BridgeHand {
//   let mut bridge_hand = BridgeHand::new(board_num);
//   for seat in Seat::iter() {
//     players[seat].receive_hand(bridge_hand.player_hand(seat));
//   }
//   bridge_hand = loop {
//     let call = players[auction.current_bidder()].get_call(&auction);
//     let auction_state = auction.make_call(call);
//     match auction_state {
//       AuctionState::InProgress(new_auction) => {
//         auction = new_auction;
//       }
//       AuctionState::Complete(completed) => {
//         break completed;
//       }
//     };
//   };
//   let play = match complete_auction {
//     CompletedAuction::Passout(passed) => {
//       return BridgeHand::Passout(passed);
//     }
//     CompletedAuction::Playable(play) => play
//   };
//   let opening_leader = play.declarer().next_seat();
//   let opening_lead = players[opening_leader].get_play(&play);
//
//   play.finish()
// }
