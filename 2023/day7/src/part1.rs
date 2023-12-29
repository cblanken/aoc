use aoc_utils::{
    read_file,
};

use std::collections::HashMap;

const CARD_RANKS: [char; 13] = [
    'A',
    'K',
    'Q',
    'J',
    'T',
    '9',
    '8',
    '7',
    '6',
    '5',
    '4',
    '3',
    '2',
];

fn get_card_rank(c: &char) -> i32 {
    for i in 0..CARD_RANKS.len() {
        if *c == CARD_RANKS[i] {
            return i as i32;
        }
    }

    -1
}

fn add_high_card_ranks(ranks: &mut Vec<i32>, frequencies: &HashMap<char, u32>) {
    let mut new_ranks: Vec<i32> = vec![];
    for (k, v) in frequencies.into_iter().filter(|(_, v)| **v == 1) {
        new_ranks.push(get_card_rank(&k));
    }

    new_ranks.sort();
    ranks.append(&mut new_ranks);
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum CamelHandType {
    FiveOfKind,
    FourOfKind,
    FullHouse,
    ThreeOfKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Debug)]
struct CamelHand<'a> {
    cards: &'a str,
    camel_type: CamelHandType,
    bid: u32,
    ranks: Vec<i32>,
}

impl CamelHand<'_> {
    fn new(cards: &str, bid: u32) -> CamelHand {
        assert!(cards.len() == 5);
        let mut hand = CamelHand { cards, camel_type: CamelHandType::HighCard, bid, ranks: vec![] };

        let mut frequency: HashMap<char, u32> = HashMap::new();
        for c in cards.chars() {
            *frequency.entry(c).or_insert(0) += 1;
        }

        if hand.is_five_of_kind(&frequency) {
            hand.camel_type = CamelHandType::FiveOfKind;
        } else if hand.is_four_of_kind(&frequency) {
            hand.camel_type = CamelHandType::FourOfKind;
        } else if hand.is_full_house(&frequency) {
            hand.camel_type = CamelHandType::FullHouse;
        } else if hand.is_three_of_kind(&frequency) {
            hand.camel_type = CamelHandType::ThreeOfKind;
        } else if hand.is_two_pair(&frequency) {
            hand.camel_type = CamelHandType::TwoPair;
        } else if hand.is_one_pair(&frequency) {
            hand.camel_type = CamelHandType::OnePair;
        } else {
            hand.ranks.clear();
            add_high_card_ranks(&mut hand.ranks, &frequency);
            hand.camel_type = CamelHandType::HighCard;
        }

        hand
    }

    fn is_five_of_kind(&mut self, card_frequencies: &HashMap<char, u32>) -> bool {
        for (k, v) in card_frequencies.into_iter() {
            if *v == 5 {
                self.ranks.push(get_card_rank(k));
                self.ranks.sort();
                return true;
            }
        }

        false
    }

    fn is_four_of_kind(&mut self, card_frequencies: &HashMap<char, u32>) -> bool {
        for (k, v) in card_frequencies.into_iter() {
            if *v == 4 {
                self.ranks.push(get_card_rank(k));
                self.ranks.sort();
                add_high_card_ranks(&mut self.ranks, card_frequencies);
                return true;
            }
        }
        false
    }

    fn is_full_house(&mut self, card_frequencies: &HashMap<char, u32>) -> bool {
        let mut has_three_group = false;
        for (k, v) in card_frequencies.into_iter() {
            if *v == 3 {
                self.ranks.push(get_card_rank(k));
                self.ranks.sort();
                has_three_group = true;
            }
        }

        if !has_three_group {
            return false;
        } else {
            for (k, v) in card_frequencies.into_iter() {
                if *v == 2 {
                    self.ranks.push(get_card_rank(k));
                    add_high_card_ranks(&mut self.ranks, card_frequencies);
                    return true;
                }
            }

            return false;
        }
    }

    fn is_three_of_kind(&mut self, card_frequencies: &HashMap<char, u32>) -> bool {
        for (k, v) in card_frequencies.into_iter() {
            if *v == 3 {
                self.ranks.push(get_card_rank(k));
                self.ranks.sort();
                add_high_card_ranks(&mut self.ranks, card_frequencies);
                return true;
            }
        }
        false
    }

    fn is_two_pair(&mut self, card_frequencies: &HashMap<char, u32>) -> bool {
        let mut pair_ranks: Vec<i32> = vec![];
        for (k, v) in card_frequencies.into_iter() {
            if *v == 2 {
                pair_ranks.push(get_card_rank(k))
            }
        }

        if pair_ranks.len() == 2 {
            for r in pair_ranks {
                self.ranks.push(r)
            }
            self.ranks.sort();
            add_high_card_ranks(&mut self.ranks, card_frequencies);
            return true;
        }
        false
    }

    fn is_one_pair(&mut self, card_frequencies: &HashMap<char, u32>) -> bool {
        for (k, v) in card_frequencies.into_iter() {
            if *v == 2 {
                self.ranks.push(get_card_rank(k));
                add_high_card_ranks(&mut self.ranks, card_frequencies);
                return true;
            }
        }
        false
    }
}

pub fn solve(filepath: &str) -> String {
    let data = read_file(filepath);

    let mut hands: Vec<CamelHand> = data.lines().map(|line| {
        let split: Vec<&str> = line.split_whitespace().collect();
        CamelHand::new(split[0].trim(), split[1].trim().parse::<u32>().unwrap())
    }).collect();


    hands.sort_unstable_by_key(|item| (item.camel_type, item.cards.chars().map(|c| get_card_rank(&c)).collect::<Vec<i32>>()));

    for h in &hands {
        match h.camel_type {
            CamelHandType::HighCard     => { },
            CamelHandType::OnePair      => { },
            CamelHandType::TwoPair      => { },
            CamelHandType::ThreeOfKind  => { },
            CamelHandType::FullHouse    => { },
            CamelHandType::FourOfKind   => { },
            CamelHandType::FiveOfKind   => { },
        }

        // dbg!(h);
    }

    hands.into_iter()
        .rev()
        .enumerate()
        .fold(0, |acc, h| acc + (h.0 + 1) * h.1.bid as usize)
        .to_string()
}
