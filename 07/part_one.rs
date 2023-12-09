use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;

const CARD_MAX_VALUE: usize = 15;
const MAX_MATCHES: usize = 6;

#[derive(PartialOrd, Ord, PartialEq, Eq, Debug)]
enum Rank {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug)]
struct Hand {
    cards: Vec<char>,
    rank: Rank,
    points: u32,
}

impl Hand {
    pub fn new(cards: &Vec<char>, points: u32, cards_values: &HashMap<char, usize>) -> Self {
        Hand {
            cards: cards.to_vec(),
            rank: Self::get_rank(cards, cards_values),
            points: points,
        }
    }

    fn get_rank(cards: &Vec<char>, cards_values_map: &HashMap<char, usize>) -> Rank {
        let mut card_count: Vec<usize> = vec![0; CARD_MAX_VALUE];
        for card in cards {
            card_count[cards_values_map[card]] += 1
        }

        let mut matches: Vec<u32> = vec![0; MAX_MATCHES];
        for i in 2..CARD_MAX_VALUE {
            matches[card_count[i]] += 1;
        }

        if matches[5] > 0 {
            return Rank::FiveOfAKind;
        } else if matches[4] > 0 {
            return Rank::FourOfAKind;
        } else if matches[3] > 0 && matches[2] > 0 {
            return Rank::FullHouse;
        } else if matches[3] > 0 {
            return Rank::ThreeOfAKind;
        } else if matches[2] == 2 {
            return Rank::TwoPair;
        } else if matches[2] == 1 {
            return Rank::OnePair;
        } else {
            return Rank::HighCard;
        }
    }
}

fn read_input() -> (Vec<Vec<char>>, Vec<u32>) {
    let input: Vec<String> = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let mut cards_str: Vec<Vec<char>> = vec![];
    let mut scores: Vec<u32> = vec![];
    for line in input {
        let splitted_line: Vec<&str> = line.split_whitespace().collect();
        cards_str.push(splitted_line[0].chars().collect());
        scores.push(splitted_line[1].parse().unwrap());
    }

    (cards_str, scores)
}

fn get_cards_values_map() -> HashMap<char, usize> {
    HashMap::from([
        ('2', 2),
        ('3', 3),
        ('4', 4),
        ('5', 5),
        ('6', 6),
        ('7', 7),
        ('8', 8),
        ('9', 9),
        ('T', 10),
        ('J', 11),
        ('Q', 12),
        ('K', 13),
        ('A', 14),
    ])
}

fn get_hands(
    cards_str: &Vec<Vec<char>>,
    points: &Vec<u32>,
    cards_values_map: &HashMap<char, usize>,
) -> Vec<Hand> {
    let mut hands: Vec<Hand> = vec![];

    for (i, _) in cards_str.iter().enumerate() {
        hands.push(Hand::new(&cards_str[i], points[i], &cards_values_map));
    }

    hands
}

fn solve(cards_str: Vec<Vec<char>>, points: Vec<u32>) -> u32 {
    let cards_values_map = get_cards_values_map();

    let mut hands = get_hands(&cards_str, &points, &cards_values_map);

    hands.sort_by(|a, b| {
        if a.rank < b.rank {
            return Ordering::Less;
        } else if a.rank > b.rank {
            return Ordering::Greater;
        } else {
            for i in 0..a.cards.len() {
                let card_a = cards_values_map[&a.cards[i]];
                let card_b = cards_values_map[&b.cards[i]];

                if card_a < card_b {
                    return Ordering::Less;
                } else if card_a > card_b {
                    return Ordering::Greater;
                }
            }
        }
        return Ordering::Equal;
    });

    let mut result = 0;
    for (i, hand) in hands.iter().enumerate() {
        result += ((i + 1) as u32) * hand.points;
    }

    result
}

fn main() {
    let (cards_str, points) = read_input();

    let result = solve(cards_str, points);

    println!("{result}");
}
