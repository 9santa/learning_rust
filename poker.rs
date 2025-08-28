use std::io::{self, Write};

const NUM_RANKS: usize = 13;
const NUM_SUITS: usize = 4;
const NUM_CARDS: usize = 5;

struct PokerHand {
    num_in_rank: Vec<usize>,
    num_in_suit: Vec<usize>,
    straight: bool,
    flush: bool,
    four: bool,
    three: bool,
    pairs: usize,
}

impl PokerHand {
    fn new() -> Self {
        Self {
            num_in_rank: vec![0; NUM_RANKS],
            num_in_suit: vec![0; NUM_SUITS],
            straight: false,
            flush: false,
            four: false,
            three: false,
            pairs: 0,
        }
    }

    fn read_cards(&mut self) {
        let mut card_exists = vec![vec![false; NUM_SUITS]; NUM_RANKS];
        self.num_in_rank.fill(0);
        self.num_in_suit.fill(0);

        let mut cards_read = 0;

        while cards_read < NUM_CARDS {
            print!("Enter a card: ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim().to_lowercase();

            if input == "0" {
                std::process::exit(0);
            }

            if input.len() != 2 {
                println!("Bad card. Ignored. Enter again.");
                continue;
            }

            let rank_ch = input.chars().nth(0).unwrap();
            let suit_ch = input.chars().nth(1).unwrap();

            let rank = match rank_ch {
                '2' => 0,
                '3' => 1,
                '4' => 2,
                '5' => 3,
                '6' => 4,
                '7' => 5,
                '8' => 6,
                '9' => 7,
                't' => 8,
                'j' => 9,
                'q' => 10,
                'k' => 11,
                'a' => 12,
                _ => {
                    println!("Bad card. Ignored. Enter again.");
                    continue;
                }
            };

            let suit = match suit_ch {
                'c' => 0,
                'd' => 1,
                'h' => 2,
                's' => 3,
                _ => {
                    println!("Bad card. Ignored. Enter again.");
                    continue;
                }
            };

            if card_exists[rank][suit] {
                println!("Duplicate card. Ignored. Enter again.");
                continue;
            }

            self.num_in_rank[rank] += 1;
            self.num_in_suit[suit] += 1;
            card_exists[rank][suit] = true;
            cards_read += 1;
        }
    }

    fn analyze_hand(&mut self) {
        self.straight = false;
        self.flush = false;
        self.four = false;
        self.three = false;
        self.pairs = 0;

        // check flush
        if self.num_in_suit.iter().any(|&count| count == NUM_CARDS) {
            self.flush = true;
        }

        // check straight
        let mut rank = 0;
        while rank < NUM_RANKS && self.num_in_rank[rank] == 0 {
            rank += 1;
        }
        let mut num_consec = 0;
        while rank < NUM_RANKS && self.num_in_rank[rank] > 0 {
            num_consec += 1;
            rank += 1;
        }
        if num_consec == NUM_CARDS {
            self.straight = true;
        }

        // check multiples
        for &count in &self.num_in_rank {
            match count {
                4 => self.four = true,
                3 => self.three = true,
                2 => self.pairs += 1,
                _ => {}
            }
        }
    }

    fn print_result(&self) {
        if self.straight && self.flush {
            println!("Straight flush");
        } else if self.four {
            println!("Four of a kind");
        } else if self.three && self.pairs == 1 {
            println!("Full house");
        } else if self.flush {
            println!("Flush");
        } else if self.straight {
            println!("Straight");
        } else if self.three {
            println!("Three of a kind");
        } else if self.pairs == 2 {
            println!("Two pairs");
        } else if self.pairs == 1 {
            println!("Pair");
        } else {
            println!("High card");
        }
        println!();
    }
}

fn main() {
    loop {
        let mut hand = PokerHand::new();
        hand.read_cards();
        hand.analyze_hand();
        hand.print_result();
    }
}
