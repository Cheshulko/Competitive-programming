#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
struct Card {
    rank: u32,
    suit: char,
}

impl Card {
    fn new(s: &str) -> Self {
        Card {
            rank: if s.len() == 2 {
                match s.chars().nth(0).unwrap() {
                    'J' => 11,
                    'Q' => 12,
                    'K' => 13,
                    'A' => 14,
                    x @ _ => x.to_digit(10).unwrap(),
                }
            } else {
                10
            },
            suit: s.chars().last().unwrap(),
        }
    }
}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            match self.rank {
                11 => "J".to_string(),
                12 => "Q".to_string(),
                13 => "K".to_string(),
                14 => "A".to_string(),
                x @ _ => x.to_string(),
            },
            self.suit
        )
    }
}

#[derive(Debug)]
struct Hand<'a> {
    hand: &'a str,
    pub cards: Vec<Card>,
}

impl<'a> std::fmt::Display for Hand<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = self
            .cards
            .iter()
            .map(|card| card.to_string())
            .collect::<Vec<String>>()
            .join(" ");
        write!(f, "{}", res)
    }
}

impl<'a> PartialEq for Hand<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl<'a> PartialOrd for Hand<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.get_rank().partial_cmp(&other.get_rank())
    }
}

impl<'a> Hand<'a> {
    fn new(s: &'a str) -> Self {
        let mut cards = s.split(" ").map(|s| Card::new(s)).collect::<Vec<Card>>();
        cards.sort();

        Hand { cards, hand: s }
    }

    pub fn get_rank(&self) -> u32 {
        let mut ranks = Vec::new();
        let mut suits = Vec::new();
        for card in &self.cards {
            ranks.push(card.rank);
            suits.push(card.suit);
        }
        ranks.sort();

        let mut is_straight = true;
        for i in 1..ranks.len() {
            if ranks[i] != ranks[i - 1] + 1 {
                is_straight = false;
                break;
            }
        }

        let is_flush = suits.iter().all(|&x| x == suits[0]);

        let num_ranks = ranks
            .iter()
            .fold(std::collections::HashMap::new(), |mut map, &rank| {
                *map.entry(rank).or_insert(0) += 1;
                map
            });

        let max_count = *num_ranks.values().max().unwrap_or(&0);

        let y = match (is_straight, is_flush, max_count) {
            (true, true, _) if ranks[0] == 10 => 1_000_000, // royal flush
            (true, true, _) => 900_000 + ranks[4],          // straight flush
            (_, _, 4) => 800_000 + ranks[4],                // four of a kind
            (_, _, 3) if num_ranks.len() == 2 => 700_000 + ranks[0], // full house | to highest-ranked triplet
            (_, true, _) => 600_000 + ranks[4],                      // flush
            (true, _, _) => 500_000 + ranks[4],                      // straight
            (_, _, 3) => 400_000 + ranks[4],                         // three of a kind
            (_, _, 2) if num_ranks.len() == 3 => {
                300_000 + num_ranks.iter().find(|x| x.1 == &2).unwrap().0
            } // two pairs
            (_, _, 2) => 200_000 + ranks[4],                         // one pair
            _ => 100_000 + ranks[4],                                 // high card
        };
        println!(
            "YY {:?} {} {} {} {} {}",
            &self.cards, ranks[4], y, is_straight, is_flush, max_count
        );
        return y;
    }
}

pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let mut hands = hands.iter().map(|h| Hand::new(h)).collect::<Vec<Hand>>();
    // println!("Sorted {:?}", hands);

    hands.sort_by(|a, b| {
        a.partial_cmp(&b).unwrap_or(std::cmp::Ordering::Equal)
        // .reverse()
    });

    let top = hands.last().unwrap().get_rank();

    println!("TOP {}", top);
    println!("hands {:?}", hands);

    let x = hands
        .iter()
        .rev()
        .filter(|h| h.get_rank() == top)
        .map(|hand| hand.hand)
        // .flat_map(|hand| hand.as_str())
        // .flat_map(|s| s.split(" "));
        .collect();

    println!("x {:?}", x);

    return x;

    // let str_vec: Vec<&str> = string_vec.iter().map(|s| s.as_str()).collect();

    // fn main() {
    //     let num_vec = vec![123, 456, 789];

    //     println!("{:?}", str_vec);
    // }

    // hands.iter().for_each(|h| println!("{}", h));
    // let x = hands
    //     .iter()
    //     .map(|hand| hand.to_string())
    //     .map(|hand| hand.to_owned().as_str());
    // unimplemented!("Out of {hands:?}, which hand wins?")
    // fn convert_vec<'a>(string_vec: Vec<String>) -> Vec<&'a str> {
    //     let mut str_vec: Vec<&'a str> = Vec::new();
    //     for s in string_vec {
    //         str_vec.push(s.as_str());
    //     }
    //     str_vec
    // }

    // return str_vec;
    // return convert_vec(string_vec);
}
