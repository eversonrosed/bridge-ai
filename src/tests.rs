use crate::game_model::bidding::{Auction, Bid, Call, Strain};
use crate::game_model::{Board, dealer, Seat};
use crate::game_model::cards::{Card, Deck, Rank, Suit};

#[test]
fn deal_hands() {
  let deck = Deck::new();
  for (_, hand) in deck.deal_hands() {
    println!("{}", hand);
  }
}

#[test]
fn make_some_bids() {
  let mut auction = Auction::new(dealer(1));
  auction.make_call(Call::Bid(Bid::from(1, Strain::Notrump)));
  auction.make_call(Call::Pass);
  auction.make_call(Call::Bid(Bid::from(3, Strain::Notrump)));
  auction.make_call(Call::Pass);
  auction.make_call(Call::Pass);
  auction.make_call(Call::Pass);
  println!("{}", auction);
  assert!(auction.is_complete());
}

#[test]
fn make_some_plays() {
  let mut auction = Auction::new(dealer(1));
  auction.make_call(Call::Bid(Bid::from(1, Strain::Notrump)));
  auction.make_call(Call::Pass);
  auction.make_call(Call::Bid(Bid::from(3, Strain::Notrump)));
  auction.make_call(Call::Pass);
  auction.make_call(Call::Pass);
  auction.make_call(Call::Pass);
  assert!(auction.is_complete());
  let mut play = auction.play().unwrap();
  assert_eq!(play.declarer(), Seat::North);
  play.make_play(Seat::East, Card::from(Suit::Spades, Rank::Queen));
  play.make_play(Seat::South, Card::from(Suit::Spades, Rank::Two));
  play.make_play(Seat::West, Card::from(Suit::Spades, Rank::Five));
  play.make_play(Seat::North, Card::from(Suit::Spades, Rank::Ace));
  println!("{:?}", play.tricks()[0].winner(Strain::Notrump));
}
