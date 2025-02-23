use cmp::Ordering::Equal;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::Iterator;
use std::{char, cmp};

#[derive(Debug, PartialEq)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    ThreeOfAKind,
    FullHouse,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandType {
    fn value(&self) -> u32 {
        match self {
            HandType::FiveOfAKind => 6000,
            HandType::FourOfAKind => 5000,
            HandType::FullHouse => 4000,
            HandType::ThreeOfAKind => 3000,
            HandType::TwoPair => 2000,
            HandType::OnePair => 1000,
            HandType::HighCard => 0,
        }
    }
}

#[derive(Debug)]
struct Hand {
    hand: Vec<char>,
    rank: u32,
    bid: u32,
    handtype: HandType,
}

impl Hand {
    fn new(htype: HandType, bid: u32, hand: Vec<char>) -> Self {
        let rank = htype.value();
        Hand {
            hand,
            rank,
            bid,
            handtype: htype,
        }
    }
}

fn card_value(card: char) -> u32 {
    match card {
        'J' => 1, //modified for part 2
        '2'..='9' => card.to_digit(10).unwrap(),
        'T' => 10,
        'Q' => 30,
        'K' => 40,
        'A' => 50,
        _ => 0,
    }
}
fn match_hand(distinct_values: u32, max_count: u32) -> HandType {
    match (distinct_values, max_count) {
        (1, 5) => HandType::FiveOfAKind,
        (2, 4) => HandType::FourOfAKind,
        (2, 3) => HandType::FullHouse,
        (3, 3) => HandType::ThreeOfAKind,
        (3, 2) => HandType::TwoPair,
        (4, 2) => HandType::OnePair,
        _ => HandType::HighCard,
    }
}

fn main() {
    let f = File::open("input").expect("File not found");
    let reader = BufReader::new(f);
    let lines: Vec<String> = reader
        .lines()
        .map(|l| l.expect("Error reading line"))
        .collect();

    let mut hands: Vec<Hand> = Vec::new();
    for line in lines.iter() {
        let mut card_counts: HashMap<char, u32> = HashMap::new();
        let hand = line.split_whitespace().nth(0).unwrap();
        let bid: u32 = line.split_whitespace().nth(1).unwrap().parse().unwrap();
        let mut joker = false;
        for card in hand.chars() {
            if card == 'J' {
                joker = true
            }
            *card_counts.entry(card).or_insert(0) += 1;
        }

        let (distinct_values, max_count) = (
            card_counts.keys().count() as u32,
            card_counts.values().cloned().max().unwrap_or(1),
        );

        let hand_type = match joker {
            true => {
                let num_jokers = card_counts.remove(&'J').unwrap();
                let joker_hand = (
                    card_counts.keys().count() as u32,
                    card_counts.values().cloned().max().unwrap_or(0) + num_jokers,
                );
                let joker_cards = match_hand(joker_hand.0, joker_hand.1);
                let std_cards = match_hand(distinct_values, max_count);
                if joker_cards.value() > std_cards.value() {
                    joker_cards
                } else {
                    std_cards
                }
            }
            false => match_hand(distinct_values, max_count),
        };
        println!("decision {:?}", hand_type);
        hands.push(Hand::new(hand_type, bid, hand.chars().collect()));
    }
    hands.sort_by(|a, b| {
        // Compare ranks first
        let rank_cmp = a.rank.cmp(&b.rank);
        if rank_cmp != Equal {
            return rank_cmp;
        }

        for (a_c, b_c) in a.hand.iter().zip(&b.hand) {
            let rank_cmp = card_value(*a_c).cmp(&card_value(*b_c));
            if rank_cmp != Equal {
                return rank_cmp;
            }
        }
        a.bid.cmp(&b.bid)
    });
    let mut total = 0u32;
    for (idx, h) in hands.into_iter().enumerate() {
        println!(
            "hand {:?} rank {} bid {} type {:?}",
            h.hand,
            idx + 1,
            h.bid,
            h.handtype
        );
        total += (1 + idx as u32) * h.bid;
    }
    println!("{}", total);
}
