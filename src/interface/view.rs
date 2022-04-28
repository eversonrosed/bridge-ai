// use cursive::{Printer, Vec2, View};
// use cursive::direction::Direction;
// use cursive::event::{Callback, Event, EventResult};
// use cursive::reexports::crossbeam_channel::Receiver;
// use cursive::view::CannotFocus;
// use crate::game_model::bidding::Auction;
// use crate::game_model::{HandResult, Seat};
// use crate::game_model::cards::{Card, PlayerHand, Rank, Suit};
// use crate::game_model::play::Play;
// use crate::interface::messages::PlayMessage;
//
// pub struct HandView {
//   hand: PlayerHand,
//   seat: Seat,
//   rx: Receiver<PlayMessage>,
//   suit_focus: Option<Suit>,
// }
//
// impl HandView {
//   fn new(hand: PlayerHand, seat: Seat, rx: Receiver<PlayMessage>) -> Self {
//     HandView { hand, seat, rx, suit_focus: None }
//   }
// }
//
// impl View for HandView {
//   fn draw(&self, printer: &Printer) {
//     let sorted = self.hand.sort();
//     let suits = vec![Suit::Spades, Suit::Hearts, Suit::Diamonds, Suit::Clubs];
//     for (i, suit) in suits.iter().enumerate() {
//       printer.print((0, i), &suit.to_string());
//       let suit_cards = sorted[*suit].iter().fold(String::new(), |mut acc, rk| {
//         acc.push(rk.rank_char());
//         acc
//       });
//       printer.print((1, i), &suit_cards);
//     }
//   }
//
//   fn required_size(&mut self, constraint: Vec2) -> Vec2 {
//     Vec2::new(28, 4)
//   }
//
//   fn on_event(&mut self, event: Event) -> EventResult {
//     match event {
//       Event::Char(c) => match c {
//         'S' | 's' => self.suit_focus = Some(Suit::Spades),
//         'H' | 'h' => self.suit_focus = Some(Suit::Hearts),
//         'D' | 'd' => self.suit_focus = Some(Suit::Diamonds),
//         'C' | 'c' => self.suit_focus = Some(Suit::Clubs),
//         c => {
//           if let (Some(suit), Ok(rank)) = (self.suit_focus, Rank::try_from(c)) {
//             let card = Card::from(suit, rank);
//             let seat = self.seat;
//             return if self.hand.has_card(card) {
//               EventResult::Consumed(Some(Callback::from_fn(move |s| {
//                 s.find_name::<PlayView>("play").unwrap().play.make_play(seat, card);
//               })))
//             } else {
//               EventResult::Ignored
//             }
//           }
//         },
//       }
//       _ => return EventResult::Ignored,
//     }
//     EventResult::Consumed(None)
//   }
//
//   fn take_focus(&mut self, _: Direction) -> Result<EventResult, CannotFocus> {
//     if let Ok(PlayMessage::YourTurn) = self.rx.try_recv() {
//       Ok(EventResult::Consumed(None))
//     } else {
//       Err(CannotFocus)
//     }
//   }
// }
//
// pub struct AuctionView {
//   auction: Auction
// }
//
// impl View for AuctionView {
//   fn draw(&self, printer: &Printer) {
//     printer.print((0, 0), &self.auction.to_string());
//   }
//
//   fn required_size(&mut self, constraint: Vec2) -> Vec2 {
//     let height = self.auction.len() + 2;
//     Vec2::new(32, height)
//   }
// }
//
// pub struct PlayView {
//   play: Play
// }
//
// impl View for PlayView {
//   fn draw(&self, printer: &Printer) {
//     if let Some(trick) = self.play.tricks().last() {
//       if let Some(north) = trick[Seat::North] {
//         printer.print((16, 1), &north.to_string());
//       }
//       if let Some(east) = trick[Seat::East] {
//         printer.print((31, 3), &east.to_string());
//       }
//       if let Some(west) = trick[Seat::West] {
//         printer.print((1, 3), &west.to_string());
//       }
//       if let Some(south) = trick[Seat::South] {
//         printer.print((16, 5), &south.to_string());
//       }
//     }
//   }
//
//   fn required_size(&mut self, constraint: Vec2) -> Vec2 {
//     Vec2::new(32, 5)
//   }
// }
//
// pub struct ResultView {
//   result: HandResult
// }
