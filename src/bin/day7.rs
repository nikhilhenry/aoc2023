use std::{cmp::Ordering, collections::HashSet, str::FromStr};

use anyhow::{anyhow, Result};
use itertools::Itertools;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
enum HandType {
    FiveKind,
    FourKind,
    FullHouse,
    ThreeKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Hand {
    val: String,
    bid: usize,
}

fn get_rank(ch: char) -> usize {
    let ranks = "J23456789TQKA";
    ranks.find(ch).expect("invalid character")
}

fn cmp_cards(me: &String, other: &String) -> Ordering {
    let me_ranked = me.chars().map(get_rank).collect_vec();
    let other_ranked = other.chars().map(get_rank).collect_vec();
    me_ranked.cmp(&other_ranked)
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let order = other.get_type().cmp(&self.get_type());
        Some(match order {
            std::cmp::Ordering::Less => order,
            std::cmp::Ordering::Equal => cmp_cards(&self.val, &other.val),
            std::cmp::Ordering::Greater => order,
        })
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).expect("pls work")
    }
}

impl FromStr for Hand {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let (val, bid) = s.split_once(" ").ok_or(anyhow!("blank string"))?;
        Ok(Hand {
            val: val.to_string(),
            bid: bid.to_string().parse()?,
        })
    }
}

impl Hand {
    fn get_type(&self) -> HandType {
        let cards = self.val.chars().collect_vec();
        let set: HashSet<char> = HashSet::from_iter(cards.clone().into_iter());

        let is_joker = set.contains(&'J');
        let card_count = set
            .into_iter()
            .map(|card| {
                (
                    card,
                    cards.clone().into_iter().filter(|c| c == &card).count(),
                )
            })
            .collect_vec();

        let card_type = if card_count.len() == 4 {
            if is_joker {
                HandType::ThreeKind
            } else {
                HandType::OnePair
            }
        } else if card_count.len() == 3 {
            if card_count
                .iter()
                .map(|(_, count)| *count)
                .filter(|count| count == &1)
                .count()
                == 2
            {
                if !is_joker {
                    HandType::ThreeKind
                } else {
                    HandType::FourKind
                }
            } else {
                if !is_joker {
                    HandType::TwoPair
                } else if card_count.contains(&('J', 2)) {
                    HandType::FourKind
                } else {
                    HandType::FullHouse
                }
            }
        } else if card_count.len() == 2 {
            if card_count
                .iter()
                .map(|(_, count)| *count)
                .filter(|count| count == &1)
                .count()
                == 1
            {
                if !is_joker {
                    HandType::FourKind
                } else {
                    HandType::FiveKind
                }
            } else {
                if !is_joker {
                    HandType::FullHouse
                } else {
                    HandType::FiveKind
                }
            }
        } else if card_count.len() == 1 {
            HandType::FiveKind
        } else {
            if !is_joker {
                HandType::HighCard
            } else {
                HandType::OnePair
            }
        };

        //println!("{:?}: {} -> {:?}", card_count, self.val, card_type);

        card_type
    }
}

fn main() -> Result<()> {
    let mut data = aoc::read_one_per_line::<Hand>("./data/day7.input")?;
    data.sort();
    for hand in data.clone() {
        println!("{} -> {:?}", hand.val, hand.get_type());
    }

    // now compute wining
    let total_winnings: usize = data
        .iter()
        .enumerate()
        .map(|(rank, hand)| (rank + 1) * hand.bid)
        .sum();

    println!("Part 1: {total_winnings}");
    Ok(())
}
