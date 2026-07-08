use crate::auction::Auction;
use crate::contract_net::Bid;

#[test]
fn auction_resolves_assignment() {
    let mut auction = Auction::new("task-1");
    auction.submit(Bid {
        agent_id: "a".into(),
        task_id: "task-1".into(),
        score: 0.7,
    });
    auction.submit(Bid {
        agent_id: "b".into(),
        task_id: "task-1".into(),
        score: 0.4,
    });
    let winner = auction.resolve().unwrap();
    assert_eq!(winner.agent_id, "a");
}

#[test]
fn auction_counts_bids() {
    let mut auction = Auction::new("t");
    auction.submit(Bid {
        agent_id: "x".into(),
        task_id: "t".into(),
        score: 1.0,
    });
    assert_eq!(auction.bid_count(), 1);
}

#[test]
fn empty_auction_resolves_none() {
    let auction = Auction::new("empty");
    assert!(auction.resolve().is_none());
}
