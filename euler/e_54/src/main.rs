use std::ops::Index;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Color {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

impl Color {
    pub fn from_char(col_char: char) -> Color {
        match col_char {
            'C' => Color::Clubs,
            'D' => Color::Diamonds,
            'H' => Color::Hearts,
            'S' => Color::Spades,
            _ => panic!("Unknown rank"),
        }
    }

    pub fn value(&self) -> usize {
        match *self {
            Color::Clubs => 0,
            Color::Diamonds => 1,
            Color::Hearts => 2,
            Color::Spades => 3,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Rank {
    pub fn from_char(rank_char: char) -> Rank {
        match rank_char {
            '2' => Rank::Two,
            '3' => Rank::Three,
            '4' => Rank::Four,
            '5' => Rank::Five,
            '6' => Rank::Six,
            '7' => Rank::Seven,
            '8' => Rank::Eight,
            '9' => Rank::Nine,
            'T' => Rank::Ten,
            'J' => Rank::Jack,
            'Q' => Rank::Queen,
            'K' => Rank::King,
            'A' => Rank::Ace,
            _ => panic!("Unknown rank"),
        }
    }
    pub fn from_value(rank: usize) -> Self {
        match rank {
            2 => Rank::Two,
            3 => Rank::Three,
            4 => Rank::Four,
            5 => Rank::Five,
            6 => Rank::Six,
            7 => Rank::Seven,
            8 => Rank::Eight,
            9 => Rank::Nine,
            10 => Rank::Ten,
            11 => Rank::Jack,
            12 => Rank::Queen,
            13 => Rank::King,
            14 => Rank::Ace,
            _ => panic!("Bad rank value"),
        }
    }

    pub fn value(&self) -> usize {
        match *self {
            Rank::Two => 2,
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Five => 5,
            Rank::Six => 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
            Rank::Nine => 9,
            Rank::Ten => 10,
            Rank::Jack => 11,
            Rank::Queen => 12,
            Rank::King => 13,
            Rank::Ace => 14,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Card {
    rank: Rank,
    color: Color,
}

impl Card {
    pub fn new(rank: Rank, color: Color) -> Self {
        Card { rank, color }
    }
    pub fn from_chars(rank_char: char, col_char: char) -> Self {
        Card::new(Rank::from_char(rank_char), Color::from_char(col_char))
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandValue {
    HighCard,
    Pair(Rank),
    TwoPairs(Rank, Rank),
    ThreeOfAKind(Rank),
    Straight(Rank),
    Flush(Rank),
    FullHouse(Rank),
    ForOfAKind(Rank),
    StraightFlush(Rank),
}

#[derive(Debug)]
pub struct Hand {
    hand: [Card; 5],
    hand_value: HandValue,
}

impl Hand {
    pub fn new(c1: Card, c2: Card, c3: Card, c4: Card, c5: Card) -> Hand {
        let mut hand_value = HandValue::HighCard;
        let mut hand: [Card; 5] = [c1, c2, c3, c4, c5];
        hand.sort();
        let mut index: [u8; 16] = [0; 16];
        let mut color_index: [u8; 4] = [0; 4];
        for card_idx in 0..5 {
            let crt_card = &hand[card_idx];
            index[crt_card.rank.value()] += 1;
            color_index[crt_card.color.value()] += 1;
            if card_idx == 4 && color_index[crt_card.color.value()] == 5 {
                //check for flush
                hand_value = HandValue::Flush(hand[4].rank);
                if hand[4].rank.value() - hand[0].rank.value() == 4 {
                    hand_value = HandValue::StraightFlush(hand[4].rank);
                }
                return Hand { hand, hand_value };
            }
        }

        //check for pairs & more
        for val_idx in 2..15 {
            let crt_val = index[val_idx];
            if crt_val == 4 {
                hand_value = HandValue::ForOfAKind(Rank::from_value(val_idx));
                break;
            } else if crt_val == 3 {
                hand_value = match hand_value {
                    HandValue::HighCard => HandValue::ThreeOfAKind(Rank::from_value(val_idx)),
                    HandValue::Pair(_) => HandValue::FullHouse(Rank::from_value(val_idx)),
                    _ => panic!("Unexpected state {:?}", hand_value),
                }
            } else if crt_val == 2 {
                hand_value = match hand_value {
                    HandValue::HighCard => HandValue::Pair(Rank::from_value(val_idx)),
                    HandValue::Pair(rank) => {
                        let old_rank = rank;
                        let new_pair_rank = Rank::from_value(val_idx);
                        if old_rank > new_pair_rank {
                            HandValue::TwoPairs(old_rank, new_pair_rank)
                        } else {
                            HandValue::TwoPairs(new_pair_rank, old_rank)
                        }
                    }
                    HandValue::ThreeOfAKind(rank) => HandValue::FullHouse(rank),
                    _ => panic!("Unexpected state {:?}", hand_value),
                }
            }
        }
        if hand_value == HandValue::HighCard {
            if hand[4].rank.value() - hand[0].rank.value() == 4 {
                hand_value = HandValue::Straight(hand[4].rank);
            }
        }
        Hand { hand, hand_value }
    }
}

impl Index<usize> for Hand {
    type Output = Card;
    fn index(&self, index: usize) -> &Card {
        &self.hand[index]
    }
}

fn compare(hand1: &Hand, hand2: &Hand) -> i8 {
    if hand1.hand_value > hand2.hand_value {
        1
    } else if hand1.hand_value < hand2.hand_value {
        -1
    } else {
        for i in (0..5).rev() {
            if hand1.hand[i].rank > hand2.hand[i].rank {
                return 1;
            } else if hand1.hand[i].rank < hand2.hand[i].rank {
                return -1;
            }
        }
        0
    }
}

fn no_main() {
    let c1 = Card::new(Rank::Two, Color::Hearts);
    let c2 = Card::new(Rank::Two, Color::Diamonds);
    let c3 = Card::new(Rank::Ace, Color::Hearts);
    let c4 = Card::new(Rank::Ace, Color::Diamonds);
    let c5 = Card::new(Rank::Ten, Color::Spades);
    let c6 = Card::new(Rank::Ten, Color::Spades);
    let hand1 = Hand::new(c1, c2, c3, c4, c5);
    let hand2 = Hand::new(c6, c2, c3, c4, c5);
    println!("Rez {:?}", compare(&hand1, &hand2));
}
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
fn main() {
    let mut h1_wins = 0;
    let f = File::open("p054_poker.txt").unwrap();
    let file = BufReader::new(&f);
    for (_, line) in file.lines().enumerate() {
        let l = line.unwrap();
        // println!("{:?}", l);
        let mut cards = Vec::<Card>::with_capacity(32);

        for card_chars in l.split(|x| (x == ' ')) {
            let char_vec: Vec<char> = card_chars.chars().collect();
            cards.push(Card::from_chars(char_vec[0], char_vec[1]));
        }
        let hand1 = Hand::new(cards[0], cards[1], cards[2], cards[3], cards[4]);
        let hand2 = Hand::new(cards[5], cards[6], cards[7], cards[8], cards[9]);
        // println!(
        //     "{:?} vs {:?}  res {:?}",
        //     hand1,
        //     hand2,
        //     compare(&hand1, &hand2)
        // );
        if compare(&hand1, &hand2) == 1 {
            h1_wins += 1;
        }        
    }
    println!("{:?}", h1_wins);
}
