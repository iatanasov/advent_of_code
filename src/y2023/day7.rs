#![allow(dead_code)]
#![allow(unused_variables)]
use std::{collections::HashMap, cmp::Ordering};

use tracing::info;
use color_eyre::Report;
use aoc_2022::utils::{Part, str_to_digits};


pub fn execute(content: String, part: Part) -> Result<(), Report> {
    info!("Running {:?} ",part);
    match part {
        Part::One => part1(content),
        Part::Two => part2(content),
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
enum Card2 {
    A,
    K,
    Q,
    T,
    C9,
    C8,
    C7,
    C6,
    C5,
    C4,
    C3,
    C2,
    J,
}
impl Card2 {
    fn from(val: char) -> Self {
        match val{
            'A' => Card2::A,
            'K' => Card2::K,
            'Q' => Card2::Q,
            'J' => Card2::J,
            'T' => Card2::T,
            '9' => Card2::C9,
            '8' => Card2::C8,
            '7' => Card2::C7,
            '6' => Card2::C6,
            '5' => Card2::C5,
            '4' => Card2::C4,
            '3' => Card2::C3,
             _ => Card2::C2,
        }
    }
    fn value(&self) -> usize {
        match &self {
            Card2::A => 14,
            Card2::K => 13,
            Card2::Q => 12,
            Card2::T => 10,
            Card2::C9 => 9,
            Card2::C8 => 8,
            Card2::C7 => 7,
            Card2::C6 => 6,
            Card2::C5 => 5,
            Card2::C4 => 4,
            Card2::C3 => 3,
            Card2::C2 => 2,
            Card2::J => 1,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
enum Card {
    A,
    K,
    Q,
    J,
    T,
    C9,
    C8,
    C7,
    C6,
    C5,
    C4,
    C3,
    C2,
}
impl Card {
    fn from(val: char) -> Self {
        match val{
            'A' => Card::A,
            'K' => Card::K,
            'Q' => Card::Q,
            'J' => Card::J,
            'T' => Card::T,
            '9' => Card::C9,
            '8' => Card::C8,
            '7' => Card::C7,
            '6' => Card::C6,
            '5' => Card::C5,
            '4' => Card::C4,
            '3' => Card::C3,
             _ => Card::C2,
        }
    }
    fn value(&self) -> usize {
        match &self {
            Card::A => 14,
            Card::K => 13,
            Card::Q => 12,
            Card::J => 11,
            Card::T => 10,
            Card::C9 => 9,
            Card::C8 => 8,
            Card::C7 => 7,
            Card::C6 => 6,
            Card::C5 => 5,
            Card::C4 => 4,
            Card::C3 => 3,
            Card::C2 => 2,
        }
    }
}

/* impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.value() > other.value() {
            return Ordering::Greater
        } else if self.value() < other.value() {
            return Ordering::Less
        }
        return Ordering::Equal
    }
}

impl PartialOrd for Card {
   fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
       return Some(self.cmp(other))
   } 
}
impl PartialEq for Card {
    fn eq(&self, other: &Self) ->bool {
        self.value() == other.value()
    }
}
*/
#[derive(Eq, Hash, PartialEq, Debug, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OncePair,
    TwoPair,
    ThreeOfKind,
    FullHouse,
    FourOfKind,
    FiveOfKind,
}

impl HandType {
    fn from(cards: &Vec<Card>) -> HandType {
        let mut hand_map: HashMap<Card,usize> = HashMap::new();
        for c in cards.iter() {
            if let Some(count) = hand_map.get_mut(c) {
                *count = *count + 1;
            } else {
                hand_map.insert(*c,1);
            }
        }
        if hand_map.len() == 1 {
            return HandType::FiveOfKind;
        } else if hand_map.len() == 2 {
            if hand_map.into_iter().find(|(k,v)| *v == 3 ) == None {
                return HandType::FourOfKind;
            }
            return HandType::FullHouse;
        } else if hand_map.len() == 3 {
            if hand_map.into_iter().find(|(k,v)| *v == 3 ) == None {
                return HandType::TwoPair;
            }
            return HandType::ThreeOfKind;
        } else if hand_map.len() == 4 {
            return HandType::OncePair;
        }
        HandType::HighCard
    }
    fn from_2(cards: &Vec<Card2>) -> HandType {
        let mut hand_map: HashMap<Card2,usize> = HashMap::new();
        let mut htype = HandType::HighCard;
        let jokers = cards.iter().filter(|c| **c == Card2::J).count();
        for c in cards.iter() {
            if let Some(count) = hand_map.get_mut(c) {
                *count = *count + 1;
            } else {
                hand_map.insert(*c,1);
            }
        }
        if hand_map.len() == 1 {
            htype = HandType::FiveOfKind;
        } else if hand_map.len() == 2 {
            if hand_map.into_iter().find(|(k,v)| *v == 3 ) == None {
                if jokers > 0 {
                    htype = HandType::FiveOfKind;
                } else {
                    htype = HandType::FourOfKind;
                }
            } else {
                if jokers == 1 {
                    htype = HandType::FourOfKind;
                } else if jokers > 1 {
                    htype = HandType::FiveOfKind;
                } else {
                    htype = HandType::FullHouse;
                }
            }
        } else if hand_map.len() == 3 {
            if hand_map.into_iter().find(|(k,v)| *v == 3 ) == None {
                if jokers == 1 {
                    htype = HandType::FullHouse;
                } else if jokers == 2 {
                    htype = HandType::FourOfKind;
                } else {
                    htype = HandType::TwoPair;
                }
            } else {
                if jokers == 1  || jokers == 3 {
                    htype = HandType::FourOfKind;
                } else if jokers == 2 {
                    htype = HandType::FiveOfKind;
                } else {
                    htype = HandType::ThreeOfKind;
                }
            }
        } else if hand_map.len() == 4 {
            if jokers == 2  || jokers == 1 {
                htype = HandType::ThreeOfKind;
            } else {
                htype = HandType::OncePair;
            }
        } else {
            if jokers > 0 {
                htype = HandType::OncePair;
            }
        }
                
        return htype
    }

}

#[derive(Debug)]
struct Hand {
    hand_type: HandType,
    cards: Vec<Card>,
    bid: usize,
    src: String,
}

impl Hand {
    fn new(hand: &str) -> Self {
        let str_hand: Vec<&str> = hand.split(" ").collect();
        let mut cards: Vec<Card> = vec![];
        for c in str_hand[0].chars() {
            cards.push(Card::from(c));
        }
        Hand {
            hand_type: HandType::from(&cards),
            cards,
            bid: str_to_digits(str_hand[1]),
            src: String::from(str_hand[0]),
        }
    }
}
impl Eq for Hand {
    
}
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let mut ord = Ordering::Equal;
        if self.hand_type != other.hand_type {
            ord =  self.hand_type.cmp(&other.hand_type);
        } else {
            for i in 0..5 {
                if self.cards[i].value() > other.cards[i].value() {
                    ord = Ordering::Greater;
                    break;
                }
                if self.cards[i].value() < other.cards[i].value() {
                    ord = Ordering::Less;
                    break;
                }
            }
        }
        return ord
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) ->bool {
        return self.cmp(other) == Ordering::Equal
    }
}
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return Some(self.cmp(&other));
    }
}

#[allow(unused_variables)]
pub fn part1(content: String) -> Result<(), Report> {
    let mut game: Vec<Hand> = vec![];
    for l in content.lines() {
        game.push(Hand::new(l));
    }
    game.sort();
    let mut total = 0;
    for (i,g) in game.into_iter().enumerate() {
        //info!("{:?} {} {} ", g, g.bid , i+1);
        total = total + (g.bid * (i+1)); 
    }
    println!("{}", total);
    Ok(())
}

#[derive(Debug)]
struct Hand2 {
    hand_type: HandType,
    cards: Vec<Card2>,
    bid: usize,
    src: String,
}

impl Hand2 {
    fn new(hand: &str) -> Self {
        let str_hand: Vec<&str> = hand.split(" ").collect();
        let mut cards: Vec<Card2> = vec![];
        for c in str_hand[0].chars() {
            cards.push(Card2::from(c));
        }
        let h_type  = HandType::from_2(&cards);
        Hand2 {
            hand_type: h_type,
            cards,
            bid: str_to_digits(str_hand[1]),
            src: String::from(str_hand[0]),
        }
    }
}
impl Eq for Hand2 {
    
}
impl Ord for Hand2 {
    fn cmp(&self, other: &Self) -> Ordering {
        let mut ord = Ordering::Equal;
        if self.hand_type != other.hand_type {
            ord =  self.hand_type.cmp(&other.hand_type);
        } else {
            for i in 0..5 {
                if self.cards[i].value() > other.cards[i].value() {
                    ord = Ordering::Greater;
                    break;
                }
                if self.cards[i].value() < other.cards[i].value() {
                    ord = Ordering::Less;
                    break;
                }
            }
        }
        return ord
    }
}

impl PartialEq for Hand2 {
    fn eq(&self, other: &Self) ->bool {
        return self.cmp(other) == Ordering::Equal
    }
}
impl PartialOrd for Hand2 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return Some(self.cmp(&other));
    }
}

#[allow(unused_variables)]
pub fn part2(content: String) -> Result<(), Report> {
    let mut game: Vec<Hand2> = vec![];
    for l in content.lines() {
        game.push(Hand2::new(l));
    }
    game.sort();
    let mut total = 0;
    for (i,g) in game.into_iter().enumerate() {
        info!("{:?} {} {} ", g, g.bid , i+1);
        total = total + (g.bid * (i+1)); 
    }
    println!("{}", total);
    Ok(())
}
