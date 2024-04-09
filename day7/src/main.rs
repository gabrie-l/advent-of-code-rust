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
}

impl Hand {
    fn new(map: HashMap<char, u32>, htype: HandType, bid: u32, hand: Vec<char>) -> Self {
        let mut max_val: u32 = 0;
        let mut max_count: u8 = 1;
        // for ch in map.keys() {
        //     if htype == HandType::HighCard {
        //         if card_val > max_val {
        //             max_val = card_val;
        //         }
        //         continue;
        //     }
        //     let count: u8 = *map.get(ch).expect("Key error") as u8;
        //     if count > 1 && count >= max_count {
        //         max_count = count;
        //         if card_val > max_val {
        //             max_val = card_val
        //         }
        //     }
        // }
        let rank = htype.value();
        Hand { hand, rank, bid }
    }
}

fn card_value(card: char) -> u32 {
    match card {
        '2'..='9' => card.to_digit(10).unwrap(),
        'T' => 10,
        'J' => 20,
        'Q' => 30,
        'K' => 40,
        'A' => 50,
        _ => 0,
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
        for card in hand.chars() {
            *card_counts.entry(card).or_insert(0) += 1;
        }
        let distinct_values = card_counts.keys().count() as u32;
        let max_count = card_counts.values().cloned().max().unwrap_or(0);

        let hand_type = match (distinct_values, max_count) {
            (1, 5) => HandType::FiveOfAKind,
            (2, 4) => HandType::FourOfAKind,
            (2, 3) => HandType::FullHouse,
            (3, 3) => HandType::ThreeOfAKind,
            (3, 2) => HandType::TwoPair,
            (4, 2) => HandType::OnePair,
            _ => HandType::HighCard,
        };
        hands.push(Hand::new(
            card_counts,
            hand_type,
            bid,
            hand.chars().collect(),
        ));
    }
    hands.sort_by(|a, b| {
        // Compare ranks first
        let rank_cmp = a.rank.cmp(&b.rank);
        if rank_cmp != Equal {
            return rank_cmp;
        }

        // Implement tiebreakers here
        // For example, if rank is equal, compare bids
        for (a_c, b_c) in a.hand.iter().zip(&b.hand) {
            let rank_cmp = card_value(*a_c).cmp(&card_value(*b_c));
            if rank_cmp != Equal {
                return rank_cmp;
            }
            // 'a_c' represents an element from 'a.hand' and 'b_c' represents the corresponding element from 'b.hand'
            // You can perform operations on 'a_c' and 'b_c' here
        }
        a.bid.cmp(&b.bid)
    });
    let mut total = 0u32;
    for (idx, h) in hands.into_iter().enumerate() {
        println!("hand {:?} rank {} bid {}", h.hand, idx + 1, h.bid);
        total += (1 + idx as u32) * h.bid;
    }
    println!("{}", total);
}
