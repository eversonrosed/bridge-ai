use cursive::{Printer, Rect, Vec2, View};
use cursive::direction::Direction;
use cursive::event::{AnyCb, Event, EventResult};
use cursive::view::{CannotFocus, Selector, ViewNotFound};
use enum_map::EnumMap;
use crate::game_model::bidding::Auction;
use crate::game_model::{Board, HandResult, Seat};
use crate::game_model::cards::{PlayerHand, Rank, Suit};
use crate::game_model::play::Play;
use crate::interface::Player;

pub struct HandView {
  hand: EnumMap<Suit, Vec<Rank>>
}

impl HandView {
  fn new(hand: &PlayerHand) -> Self {
    HandView { hand: hand.sort() }
  }
}

impl View for HandView {
  fn draw(&self, printer: &Printer) {
    let suits = vec![Suit::Spades, Suit::Hearts, Suit::Diamonds, Suit::Clubs];
    for (i, suit) in suits.iter().enumerate() {
      printer.print((0, i), &suit.to_string());
      let suit_cards = self.hand[*suit].iter().fold(String::new(), |mut acc, rk| {
        acc.push(rk.rank_char());
        acc
      });
      printer.print((1, i), &suit_cards);
    }
  }

  fn required_size(&mut self, constraint: Vec2) -> Vec2 {
    Vec2::new(28, 4)
  }

  fn on_event(&mut self, _: Event) -> EventResult {
    todo!()
  }

  fn take_focus(&mut self, _: Direction) -> Result<EventResult, CannotFocus> {
    Ok(EventResult::Consumed(None))
  }
}

pub struct AuctionView {
  auction: Auction
}

impl View for AuctionView {
  fn draw(&self, printer: &Printer) {
    printer.print((0, 0), &self.auction.to_string());
  }

  fn required_size(&mut self, constraint: Vec2) -> Vec2 {
    let height = self.auction.len() + 2;
    Vec2::new(32, height)
  }
}

pub struct PlayView {
  play: Play
}

impl View for PlayView {
  fn draw(&self, printer: &Printer) {
    if let Some(trick) = self.play.tricks().last() {
      if let Some(north) = trick[Seat::North] {
        printer.print((16, 1), &north.to_string());
      }
      if let Some(east) = trick[Seat::East] {
        printer.print((31, 3), &east.to_string());
      }
      if let Some(west) = trick[Seat::West] {
        printer.print((1, 3), &west.to_string());
      }
      if let Some(south) = trick[Seat::South] {
        printer.print((16, 5), &south.to_string());
      }
    }
  }

  fn required_size(&mut self, constraint: Vec2) -> Vec2 {
    Vec2::new(32, 5)
  }
}

pub struct ResultView {
  result: HandResult
}
