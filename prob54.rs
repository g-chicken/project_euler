use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let matches_players: [[[Card; 5]; 2]; 1000] = match read_file() {
        Ok(mp) => mp,
        Err(error) => panic!("fail to read file: {:?}", error),
    };

    let mut player_one_win: u32 = 0;

    for match_players in matches_players {
        let player_one_result: PokerResult = new_poker_result(&match_players[0]);
        let player_two_result: PokerResult = new_poker_result(&match_players[1]);

        if player_one_result > player_two_result {
            player_one_win += 1;

            continue;
        }
    }

    println!("{}", player_one_win);
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum SuitType {
    Spade,
    Heart,
    Diamond,
    Club,
}

fn new_suit_type(ch: char) -> SuitType {
    match ch {
        'S' => SuitType::Spade,
        'H' => SuitType::Heart,
        'D' => SuitType::Diamond,
        'C' => SuitType::Club,
        _ => panic!("unknown suit type (args: {})", ch),
    }
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Copy, Clone, Hash)]
enum CardNumber {
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

impl CardNumber {
    fn next_number(&self) -> CardNumber {
        match self {
            CardNumber::Two => CardNumber::Three,
            CardNumber::Three => CardNumber::Four,
            CardNumber::Four => CardNumber::Five,
            CardNumber::Five => CardNumber::Six,
            CardNumber::Six => CardNumber::Seven,
            CardNumber::Seven => CardNumber::Eight,
            CardNumber::Eight => CardNumber::Nine,
            CardNumber::Nine => CardNumber::Ten,
            CardNumber::Ten => CardNumber::Jack,
            CardNumber::Jack => CardNumber::Queen,
            CardNumber::Queen => CardNumber::King,
            CardNumber::King => CardNumber::Ace,
            CardNumber::Ace => CardNumber::Two,
        }
    }
}

fn new_card_number(ch: char) -> CardNumber {
    match ch {
        '2' => CardNumber::Two,
        '3' => CardNumber::Three,
        '4' => CardNumber::Four,
        '5' => CardNumber::Five,
        '6' => CardNumber::Six,
        '7' => CardNumber::Seven,
        '8' => CardNumber::Eight,
        '9' => CardNumber::Nine,
        'T' => CardNumber::Ten,
        'J' => CardNumber::Jack,
        'Q' => CardNumber::Queen,
        'K' => CardNumber::King,
        'A' => CardNumber::Ace,
        _ => panic!("unknown card number (args: {})", ch),
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct Card {
    number: CardNumber,
    suit: SuitType,
}

fn new_card(s: String) -> Card {
    if s.chars().count() < 2 {
        panic!("invalid string (args: {})", s)
    }

    let mut chars = s.chars();

    Card {
        number: new_card_number(chars.next().unwrap()),
        suit: new_suit_type(chars.next().unwrap()),
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
enum Hand {
    HighCard,
    OnePair,
    TwoPair,
    ThreeCard,
    Straight,
    Flush,
    FullHouse,
    FourCard,
    StraightFlush,
    RoyalFlush,
}

#[derive(Debug, PartialEq, PartialOrd)]
struct PokerResult {
    hand: Hand,
    values: Vec<CardNumber>, // sorted
}

fn new_poker_result(cards: &[Card; 5]) -> PokerResult {
    let mut values: Vec<CardNumber> = Vec::<CardNumber>::new();

    for card in cards {
        values.push(card.number);
    }

    values.sort_by(|a, b| b.cmp(a));

    let all_same_suit: bool = is_all_same_suit(cards);
    let straight: bool = is_straight(cards);

    if all_same_suit
        && straight
        && values
            == vec![
                CardNumber::Ace,
                CardNumber::King,
                CardNumber::Queen,
                CardNumber::Jack,
                CardNumber::Ten,
            ]
    {
        return PokerResult {
            hand: Hand::RoyalFlush,
            values: values,
        };
    } else if all_same_suit && straight {
        return PokerResult {
            hand: Hand::StraightFlush,
            values: values,
        };
    }

    let same_number_map: HashMap<CardNumber, u8> = get_same_number(cards);

    if same_number_map.values().find(|&&c| c == 4) != None {
        let four: CardNumber = *same_number_map
            .keys()
            .find(|&k| *same_number_map.get(k).unwrap() == 4)
            .unwrap();
        let rest_card: CardNumber = *same_number_map
            .keys()
            .find(|&k| *same_number_map.get(k).unwrap() != 4)
            .unwrap();

        return PokerResult {
            hand: Hand::FourCard,
            values: vec![four, rest_card],
        };
    }

    if same_number_map.values().find(|&&c| c == 3) != None
        && same_number_map.values().find(|&&c| c == 2) != None
    {
        let three: CardNumber = *same_number_map
            .keys()
            .find(|&k| *same_number_map.get(k).unwrap() == 3)
            .unwrap();
        let two: CardNumber = *same_number_map
            .keys()
            .find(|&k| *same_number_map.get(k).unwrap() == 2)
            .unwrap();

        return PokerResult {
            hand: Hand::FullHouse,
            values: vec![three, two],
        };
    }

    if all_same_suit {
        return PokerResult {
            hand: Hand::Flush,
            values: values,
        };
    }

    if straight {
        return PokerResult {
            hand: Hand::Straight,
            values: values,
        };
    }

    if same_number_map.values().find(|&&c| c == 3) != None {
        let three: CardNumber = *same_number_map
            .keys()
            .find(|&k| *same_number_map.get(k).unwrap() == 3)
            .unwrap();
        let mut rest_cards: Vec<CardNumber> = Vec::<CardNumber>::new();

        for (k, v) in same_number_map.iter() {
            if *v != 3 {
                rest_cards.push(*k);
            }
        }

        rest_cards.sort_by(|a, b| b.cmp(a));
        rest_cards.insert(0, three);

        return PokerResult {
            hand: Hand::ThreeCard,
            values: rest_cards,
        };
    }

    if same_number_map.values().filter(|&&c| c == 2).count() == 2 {
        let rest_card: CardNumber = *same_number_map
            .keys()
            .find(|&k| *same_number_map.get(k).unwrap() != 2)
            .unwrap();
        let mut pair: Vec<CardNumber> = Vec::<CardNumber>::new();

        for (k, v) in same_number_map.iter() {
            if *v == 2 {
                pair.push(*k);
            }
        }

        pair.sort_by(|a, b| b.cmp(a));
        pair.push(rest_card);

        return PokerResult {
            hand: Hand::TwoPair,
            values: pair,
        };
    }

    if same_number_map.values().filter(|&&c| c == 2).count() == 1 {
        let pair: CardNumber = *same_number_map
            .keys()
            .find(|&k| *same_number_map.get(k).unwrap() == 2)
            .unwrap();
        let mut rest_cards: Vec<CardNumber> = Vec::<CardNumber>::new();

        for (k, v) in same_number_map.iter() {
            if *v != 2 {
                rest_cards.push(*k);
            }
        }

        rest_cards.sort_by(|a, b| b.cmp(a));
        rest_cards.insert(0, pair);

        return PokerResult {
            hand: Hand::OnePair,
            values: rest_cards,
        };
    }

    return PokerResult {
        hand: Hand::HighCard,
        values: values,
    };
}

fn get_same_number(cards: &[Card; 5]) -> HashMap<CardNumber, u8> {
    let mut same_number: HashMap<CardNumber, u8> = HashMap::<CardNumber, u8>::new();

    for card in cards {
        if !same_number.contains_key(&card.number) {
            same_number.insert(card.number, 1);

            continue;
        }

        let c: u8 = *same_number.get(&card.number).unwrap();
        same_number.insert(card.number, c + 1);
    }

    same_number
}

fn is_all_same_suit(cards: &[Card; 5]) -> bool {
    for card in cards {
        if card.suit != cards[0].suit {
            return false;
        }
    }

    true
}

fn is_straight(cards: &[Card; 5]) -> bool {
    let mut numbers: Vec<CardNumber> = Vec::<CardNumber>::new();

    for card in cards {
        numbers.push(card.number);
    }

    if numbers.contains(&CardNumber::Ace) {
        let straight1: Vec<CardNumber> = vec![
            CardNumber::Ace,
            CardNumber::King,
            CardNumber::Queen,
            CardNumber::Jack,
            CardNumber::Ten,
        ];
        let straight2: Vec<CardNumber> = vec![
            CardNumber::Ace,
            CardNumber::Five,
            CardNumber::Four,
            CardNumber::Three,
            CardNumber::Two,
        ];

        numbers.sort_by(|a, b| b.cmp(a));

        return numbers
            .iter()
            .zip(&straight1)
            .filter(|&(a, b)| a == b)
            .count()
            == 5
            || numbers
                .iter()
                .zip(&straight2)
                .filter(|&(a, b)| a == b)
                .count()
                == 5;
    }

    let mut count: u8 = 0;
    let min: CardNumber = *numbers.iter().min().unwrap();
    let mut next: CardNumber = min.next_number();

    while numbers.contains(&next) {
        count += 1;
        next = next.next_number();
    }

    count == 4
}

fn read_file() -> Result<[[[Card; 5]; 2]; 1000], Box<dyn std::error::Error>> {
    let mut matches_players: [[[Card; 5]; 2]; 1000] = [[[Card {
        number: CardNumber::Two,
        suit: SuitType::Spade,
    }; 5]; 2]; 1000];
    let mut line_num = 0;

    let buf = BufReader::new(File::open("./prob54.txt").expect("open failed"));

    for line in buf.lines() {
        let l = line.expect("lines failed");
        let split: Vec<&str> = l.split(' ').collect();

        for i in 0..5 {
            matches_players[line_num][0][i] = new_card(split[i].to_string());
        }

        for i in 5..10 {
            matches_players[line_num][1][i - 5] = new_card(split[i].to_string());
        }

        line_num += 1;
    }

    Ok(matches_players)
}

#[test]
fn test_new_suit_type() {
    assert_eq!(new_suit_type('S'), SuitType::Spade);
    assert_eq!(new_suit_type('H'), SuitType::Heart);
    assert_eq!(new_suit_type('D'), SuitType::Diamond);
    assert_eq!(new_suit_type('C'), SuitType::Club);
}

#[test]
fn test_new_card_number() {
    assert_eq!(new_card_number('2'), CardNumber::Two);
    assert_eq!(new_card_number('3'), CardNumber::Three);
    assert_eq!(new_card_number('4'), CardNumber::Four);
    assert_eq!(new_card_number('5'), CardNumber::Five);
    assert_eq!(new_card_number('6'), CardNumber::Six);
    assert_eq!(new_card_number('7'), CardNumber::Seven);
    assert_eq!(new_card_number('8'), CardNumber::Eight);
    assert_eq!(new_card_number('9'), CardNumber::Nine);
    assert_eq!(new_card_number('T'), CardNumber::Ten);
    assert_eq!(new_card_number('J'), CardNumber::Jack);
    assert_eq!(new_card_number('Q'), CardNumber::Queen);
    assert_eq!(new_card_number('K'), CardNumber::King);
    assert_eq!(new_card_number('A'), CardNumber::Ace);
}

#[test]
fn test_new_card() {
    assert_eq!(
        new_card("AS".to_string()),
        Card {
            number: CardNumber::Ace,
            suit: SuitType::Spade,
        }
    );
    assert_eq!(
        new_card("TH".to_string()),
        Card {
            number: CardNumber::Ten,
            suit: SuitType::Heart,
        }
    );
    assert_eq!(
        new_card("KC".to_string()),
        Card {
            number: CardNumber::King,
            suit: SuitType::Club,
        }
    );
    assert_eq!(
        new_card("6D".to_string()),
        Card {
            number: CardNumber::Six,
            suit: SuitType::Diamond,
        }
    );
}

#[test]
fn test_result_cmp() {
    assert!(
        PokerResult {
            hand: Hand::TwoPair,
            values: vec![CardNumber::King],
        } < PokerResult {
            hand: Hand::FullHouse,
            values: vec![CardNumber::Seven, CardNumber::Three],
        }
    );
    assert!(
        PokerResult {
            hand: Hand::Straight,
            values: vec![
                CardNumber::Ten,
                CardNumber::Nine,
                CardNumber::Eight,
                CardNumber::Seven,
                CardNumber::Six
            ],
        } < PokerResult {
            hand: Hand::Flush,
            values: vec![
                CardNumber::Ace,
                CardNumber::Jack,
                CardNumber::Five,
                CardNumber::Four,
                CardNumber::Two
            ],
        }
    );
    assert!(
        PokerResult {
            hand: Hand::Flush,
            values: vec![
                CardNumber::Ten,
                CardNumber::Nine,
                CardNumber::Eight,
                CardNumber::Seven,
                CardNumber::Six
            ],
        } < PokerResult {
            hand: Hand::Flush,
            values: vec![
                CardNumber::Ace,
                CardNumber::Jack,
                CardNumber::Five,
                CardNumber::Four,
                CardNumber::Two
            ],
        }
    );
    assert!(
        PokerResult {
            hand: Hand::HighCard,
            values: vec![
                CardNumber::Queen,
                CardNumber::Nine,
                CardNumber::Five,
                CardNumber::Four,
                CardNumber::Two,
            ],
        } < PokerResult {
            hand: Hand::HighCard,
            values: vec![
                CardNumber::Queen,
                CardNumber::Nine,
                CardNumber::Five,
                CardNumber::Four,
                CardNumber::Three,
            ],
        }
    );
}

#[test]
fn test_get_same_number() {
    let mut cards: [Card; 5] = [
        Card {
            number: CardNumber::Six,
            suit: SuitType::Diamond,
        },
        Card {
            number: CardNumber::King,
            suit: SuitType::Club,
        },
        Card {
            number: CardNumber::Two,
            suit: SuitType::Club,
        },
        Card {
            number: CardNumber::Three,
            suit: SuitType::Heart,
        },
        Card {
            number: CardNumber::Five,
            suit: SuitType::Spade,
        },
    ];
    let mut result: HashMap<CardNumber, u8> = HashMap::<CardNumber, u8>::from([
        (CardNumber::Six, 1),
        (CardNumber::King, 1),
        (CardNumber::Two, 1),
        (CardNumber::Three, 1),
        (CardNumber::Five, 1),
    ]);

    assert_eq!(get_same_number(&cards), result);

    cards[3] = Card {
        number: CardNumber::King,
        suit: SuitType::Diamond,
    };
    result.remove(&CardNumber::Three);
    result.insert(CardNumber::King, 2);
    assert_eq!(get_same_number(&cards), result);

    cards[0] = Card {
        number: CardNumber::Two,
        suit: SuitType::Heart,
    };
    result.remove(&CardNumber::Six);
    result.insert(CardNumber::Two, 2);
    assert_eq!(get_same_number(&cards), result);

    cards[4] = Card {
        number: CardNumber::King,
        suit: SuitType::Spade,
    };
    result.remove(&CardNumber::Five);
    result.insert(CardNumber::King, 3);
    assert_eq!(get_same_number(&cards), result);

    cards[2] = Card {
        number: CardNumber::King,
        suit: SuitType::Heart,
    };
    result.remove(&CardNumber::Five);
    result.insert(CardNumber::King, 4);
    result.insert(CardNumber::Two, 1);
    assert_eq!(get_same_number(&cards), result);
}

#[test]
fn test_is_all_same_suit() {
    let mut cards: [Card; 5] = [
        Card {
            number: CardNumber::Six,
            suit: SuitType::Diamond,
        },
        Card {
            number: CardNumber::King,
            suit: SuitType::Club,
        },
        Card {
            number: CardNumber::Two,
            suit: SuitType::Club,
        },
        Card {
            number: CardNumber::Three,
            suit: SuitType::Heart,
        },
        Card {
            number: CardNumber::Five,
            suit: SuitType::Spade,
        },
    ];

    assert!(!is_all_same_suit(&cards));

    cards[2].suit = SuitType::Diamond;
    assert!(!is_all_same_suit(&cards));

    cards[3].suit = SuitType::Diamond;
    assert!(!is_all_same_suit(&cards));

    cards[1].suit = SuitType::Diamond;
    assert!(!is_all_same_suit(&cards));

    cards[4].suit = SuitType::Diamond;
    assert!(is_all_same_suit(&cards));
}

#[test]
fn test_is_straight() {
    let mut cards: [Card; 5] = [
        Card {
            number: CardNumber::Six,
            suit: SuitType::Diamond,
        },
        Card {
            number: CardNumber::King,
            suit: SuitType::Club,
        },
        Card {
            number: CardNumber::Two,
            suit: SuitType::Club,
        },
        Card {
            number: CardNumber::Three,
            suit: SuitType::Heart,
        },
        Card {
            number: CardNumber::Five,
            suit: SuitType::Spade,
        },
    ];
    assert!(!is_straight(&cards));

    cards[0].number = CardNumber::Ten;
    cards[1].number = CardNumber::Queen;
    cards[2].number = CardNumber::Ace;
    cards[3].number = CardNumber::Jack;
    cards[4].number = CardNumber::King;
    assert!(is_straight(&cards));

    cards[0].number = CardNumber::Three;
    cards[1].number = CardNumber::Five;
    cards[2].number = CardNumber::Ace;
    cards[3].number = CardNumber::Two;
    cards[4].number = CardNumber::Four;
    assert!(is_straight(&cards));

    cards[0].number = CardNumber::Eight;
    cards[1].number = CardNumber::Five;
    cards[2].number = CardNumber::Three;
    cards[3].number = CardNumber::Two;
    cards[4].number = CardNumber::Ace;
    assert!(!is_straight(&cards));

    cards[0].number = CardNumber::Two;
    cards[1].number = CardNumber::Queen;
    cards[2].number = CardNumber::Ace;
    cards[3].number = CardNumber::Jack;
    cards[4].number = CardNumber::King;
    assert!(!is_straight(&cards));
}

#[test]
fn test_new_poker_result_royal_flush() {
    let mut cards: [Card; 5] = [
        Card {
            number: CardNumber::King,
            suit: SuitType::Diamond,
        },
        Card {
            number: CardNumber::Jack,
            suit: SuitType::Diamond,
        },
        Card {
            number: CardNumber::Ace,
            suit: SuitType::Diamond,
        },
        Card {
            number: CardNumber::Ten,
            suit: SuitType::Diamond,
        },
        Card {
            number: CardNumber::Queen,
            suit: SuitType::Diamond,
        },
    ];
    let result: PokerResult = PokerResult {
        hand: Hand::RoyalFlush,
        values: vec![
            CardNumber::Ace,
            CardNumber::King,
            CardNumber::Queen,
            CardNumber::Jack,
            CardNumber::Ten,
        ],
    };

    assert_eq!(new_poker_result(&cards), result);

    cards[0].suit = SuitType::Spade;
    cards[1].suit = SuitType::Spade;
    cards[2].suit = SuitType::Spade;
    cards[3].suit = SuitType::Spade;
    cards[4].suit = SuitType::Spade;
    assert_eq!(new_poker_result(&cards), result);
}

#[test]
fn test_new_poker_result_straight_flush() {
    let mut cards: [Card; 5] = [
        Card {
            number: CardNumber::Three,
            suit: SuitType::Club,
        },
        Card {
            number: CardNumber::Four,
            suit: SuitType::Club,
        },
        Card {
            number: CardNumber::Ace,
            suit: SuitType::Club,
        },
        Card {
            number: CardNumber::Two,
            suit: SuitType::Club,
        },
        Card {
            number: CardNumber::Five,
            suit: SuitType::Club,
        },
    ];
    let mut result: PokerResult = PokerResult {
        hand: Hand::StraightFlush,
        values: vec![
            CardNumber::Ace,
            CardNumber::Five,
            CardNumber::Four,
            CardNumber::Three,
            CardNumber::Two,
        ],
    };

    assert_eq!(new_poker_result(&cards), result);

    cards[0].suit = SuitType::Heart;
    cards[1].suit = SuitType::Heart;
    cards[2].suit = SuitType::Heart;
    cards[3].suit = SuitType::Heart;
    cards[4].suit = SuitType::Heart;
    assert_eq!(new_poker_result(&cards), result);

    cards[1].number = CardNumber::Five;
    cards[2].number = CardNumber::Four;
    cards[4].number = CardNumber::Six;
    result.values[0] = CardNumber::Six;
    result.values[1] = CardNumber::Five;
    result.values[2] = CardNumber::Four;
    assert_eq!(new_poker_result(&cards), result);
}

#[test]
fn test_new_poker_result_four_card() {
    let cards: [Card; 5] = [
        Card {
            number: CardNumber::Three,
            suit: SuitType::Heart,
        },
        Card {
            number: CardNumber::Three,
            suit: SuitType::Club,
        },
        Card {
            number: CardNumber::Three,
            suit: SuitType::Diamond,
        },
        Card {
            number: CardNumber::Jack,
            suit: SuitType::Diamond,
        },
        Card {
            number: CardNumber::Three,
            suit: SuitType::Spade,
        },
    ];
    let result: PokerResult = PokerResult {
        hand: Hand::FourCard,
        values: vec![CardNumber::Three, CardNumber::Jack],
    };

    assert_eq!(new_poker_result(&cards), result);
}

#[test]
fn test_new_poker_result_full_house() {
    let cards: [Card; 5] = [
        Card {
            number: CardNumber::Three,
            suit: SuitType::Heart,
        },
        Card {
            number: CardNumber::Three,
            suit: SuitType::Club,
        },
        Card {
            number: CardNumber::Jack,
            suit: SuitType::Spade,
        },
        Card {
            number: CardNumber::Jack,
            suit: SuitType::Diamond,
        },
        Card {
            number: CardNumber::Three,
            suit: SuitType::Spade,
        },
    ];
    let result: PokerResult = PokerResult {
        hand: Hand::FullHouse,
        values: vec![CardNumber::Three, CardNumber::Jack],
    };

    assert_eq!(new_poker_result(&cards), result);
}

#[test]
fn test_new_poker_result_flush() {
    let mut cards: [Card; 5] = [
        Card {
            number: CardNumber::Eight,
            suit: SuitType::Heart,
        },
        Card {
            number: CardNumber::Five,
            suit: SuitType::Heart,
        },
        Card {
            number: CardNumber::Queen,
            suit: SuitType::Heart,
        },
        Card {
            number: CardNumber::Ace,
            suit: SuitType::Heart,
        },
        Card {
            number: CardNumber::Four,
            suit: SuitType::Heart,
        },
    ];
    let result: PokerResult = PokerResult {
        hand: Hand::Flush,
        values: vec![
            CardNumber::Ace,
            CardNumber::Queen,
            CardNumber::Eight,
            CardNumber::Five,
            CardNumber::Four,
        ],
    };

    assert_eq!(new_poker_result(&cards), result);

    cards[0].suit = SuitType::Spade;
    cards[1].suit = SuitType::Spade;
    cards[2].suit = SuitType::Spade;
    cards[3].suit = SuitType::Spade;
    cards[4].suit = SuitType::Spade;
    assert_eq!(new_poker_result(&cards), result);
}

#[test]
fn test_new_poker_result_straight() {
    let cards: [Card; 5] = [
        Card {
            number: CardNumber::Eight,
            suit: SuitType::Heart,
        },
        Card {
            number: CardNumber::Ten,
            suit: SuitType::Club,
        },
        Card {
            number: CardNumber::Jack,
            suit: SuitType::Heart,
        },
        Card {
            number: CardNumber::Nine,
            suit: SuitType::Heart,
        },
        Card {
            number: CardNumber::Seven,
            suit: SuitType::Heart,
        },
    ];
    let result: PokerResult = PokerResult {
        hand: Hand::Straight,
        values: vec![
            CardNumber::Jack,
            CardNumber::Ten,
            CardNumber::Nine,
            CardNumber::Eight,
            CardNumber::Seven,
        ],
    };

    assert_eq!(new_poker_result(&cards), result);
}

#[test]
fn test_new_poker_result_three_card() {
    let cards: [Card; 5] = [
        Card {
            number: CardNumber::Eight,
            suit: SuitType::Heart,
        },
        Card {
            number: CardNumber::Ten,
            suit: SuitType::Club,
        },
        Card {
            number: CardNumber::Jack,
            suit: SuitType::Heart,
        },
        Card {
            number: CardNumber::Ten,
            suit: SuitType::Heart,
        },
        Card {
            number: CardNumber::Ten,
            suit: SuitType::Spade,
        },
    ];
    let result: PokerResult = PokerResult {
        hand: Hand::ThreeCard,
        values: vec![CardNumber::Ten, CardNumber::Jack, CardNumber::Eight],
    };

    assert_eq!(new_poker_result(&cards), result);
}

#[test]
fn test_new_poker_result_two_pair() {
    let cards: [Card; 5] = [
        Card {
            number: CardNumber::Eight,
            suit: SuitType::Heart,
        },
        Card {
            number: CardNumber::Ten,
            suit: SuitType::Club,
        },
        Card {
            number: CardNumber::Jack,
            suit: SuitType::Heart,
        },
        Card {
            number: CardNumber::Jack,
            suit: SuitType::Diamond,
        },
        Card {
            number: CardNumber::Ten,
            suit: SuitType::Spade,
        },
    ];
    let result: PokerResult = PokerResult {
        hand: Hand::TwoPair,
        values: vec![CardNumber::Jack, CardNumber::Ten, CardNumber::Eight],
    };

    assert_eq!(new_poker_result(&cards), result);
}

#[test]
fn test_new_poker_result_one_pair() {
    let cards: [Card; 5] = [
        Card {
            number: CardNumber::Eight,
            suit: SuitType::Heart,
        },
        Card {
            number: CardNumber::Ten,
            suit: SuitType::Club,
        },
        Card {
            number: CardNumber::Jack,
            suit: SuitType::Heart,
        },
        Card {
            number: CardNumber::Jack,
            suit: SuitType::Diamond,
        },
        Card {
            number: CardNumber::Ace,
            suit: SuitType::Spade,
        },
    ];
    let result: PokerResult = PokerResult {
        hand: Hand::OnePair,
        values: vec![
            CardNumber::Jack,
            CardNumber::Ace,
            CardNumber::Ten,
            CardNumber::Eight,
        ],
    };

    assert_eq!(new_poker_result(&cards), result);
}

#[test]
fn test_new_poker_result_high_card() {
    let cards: [Card; 5] = [
        Card {
            number: CardNumber::Eight,
            suit: SuitType::Heart,
        },
        Card {
            number: CardNumber::Ten,
            suit: SuitType::Club,
        },
        Card {
            number: CardNumber::Jack,
            suit: SuitType::Heart,
        },
        Card {
            number: CardNumber::Two,
            suit: SuitType::Diamond,
        },
        Card {
            number: CardNumber::Ace,
            suit: SuitType::Spade,
        },
    ];
    let result: PokerResult = PokerResult {
        hand: Hand::HighCard,
        values: vec![
            CardNumber::Ace,
            CardNumber::Jack,
            CardNumber::Ten,
            CardNumber::Eight,
            CardNumber::Two,
        ],
    };

    assert_eq!(new_poker_result(&cards), result);
}
