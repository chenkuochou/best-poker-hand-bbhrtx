/* Read cards e.g. Card(5, 'H') */
#[derive(Debug, Ord, Eq, PartialOrd, PartialEq)]
struct Card(u8, char);
impl Card {
    fn new(card_str: &str) -> Self {
        let bytes = card_str.as_bytes();
        let rank: u8 = match &bytes[0] {
            b'A' => 14,
            b'K' => 13,
            b'Q' => 12,
            b'J' => 11,
            b'1' => 10,
            c => c - b'0',
        };
        let suit = bytes[bytes.len() - 1];
        Card(rank, suit as char)
    }
}


/* Hands of poker patterns (rules to compare) */
#[derive(Debug, Ord, Eq, PartialOrd, PartialEq)]
enum Hand {
    HighCard(u8, u8, u8, u8, u8),
    OnePair(u8, u8, u8, u8),
    TwoPair(u8, u8, u8),
    ThreeOfAKind(u8, u8, u8),
    Straight(u8),
    Flush(u8, u8, u8, u8, u8),
    FullHouse(u8, u8),
    FourOfAKind(u8, u8),
    StraightFlush(u8),
}
impl Hand {
    fn new(cards: &mut [Card]) -> Self {
        use Hand::*;


        /* Sorting */
        cards.sort_by(|x, y| y.cmp(x));
        // println!("sorted:{:?}", cards);
        // [Card(8, 'D'), Card(5, 'H'), Card(4, 'S'), Card(4, 'H'), Card(4, 'C')]


        /* Sorted parttern (number, count) for each card */
        let mut number_count = (1..=14)
            .map(|n| (n, cards.iter().filter(|i| i.0 == n).count() as u8))
            .filter(|(_, count)| *count != 0)
            .collect::<Vec<_>>();


        number_count.sort_unstable_by(|x, y| (y.1, y.0).cmp(&(x.1, x.0)));
        // I asked for the above structure 
        // https://stackoverflow.com/questions/70919627/how-to-sort-items-inside-a-vector-in-rust


        let nc = number_count.as_slice();
        // println!("nc:{:?}", nc);
        // [(4, 3), (8, 1), (5, 1)]


        /* Match patterns by rank */
        let straight = cards.windows(2).all(|w| w[0].0 == w[1].0 + 1);
        let flush = cards.windows(2).all(|w| w[0].1 == w[1].1);

        match (straight, flush, nc[0].1, nc[1].1) {
            (true, true, _, _) => StraightFlush(cards[4].0),
            (_, _, 4, _) => FourOfAKind(nc[0].0, nc[1].0),
            (_, _, 3, 2) => FullHouse(nc[0].0, nc[1].0),
            (_, true, _, _) => Flush(cards[0].0, cards[1].0, cards[2].0, cards[3].0, cards[4].0),
            (true, _, _, _) => Straight(match cards[0].0 {
                14 => 5,
                c => c,
            }), // ISSUE?
            (_, _, 3, _) => ThreeOfAKind(nc[0].0, nc[1].0, nc[2].0),
            (_, _, 2, 2) => TwoPair(nc[0].0, nc[1].0, nc[2].0),
            (_, _, 2, _) => OnePair(nc[0].0, nc[1].0, nc[2].0, nc[3].0),
            _ => HighCard(cards[0].0, cards[1].0, cards[2].0, cards[3].0, cards[4].0),
        }
    }
}

pub fn winning_hands<'a>(hands: &[&'a str]) -> Option<Vec<&'a str>> {
    // The returned referecne 'Option<>' will also be valid for the length of 'hands'
    // CANNOT return a referecne to somethig created inside the fucntion!

    // println!("original hands:{:?}", hands);
    // ["4S 5H 4C 8D 4H", "4D AH 3S 2D 5C"]


    /* Feed Hand with Card */
    let mut hands = hands
        .iter()
        .map(|h| {
            let hand = Hand::new(
                h.split(|c: char| c.is_whitespace())
                 .map(|c| Card::new(c))
                 .collect::<Vec<_>>()
                 .as_mut_slice(),
            );
            (hand, *h)
        })
        .collect::<Vec<_>>();
    hands.sort_by(|x, y| y.cmp(x));
    // println!("hands:{:?}", hands);
    // [(ThreeOfAKind(4, 8, 5), "4S 5H 4C 8D 4H"), (HighCard(14, 5, 4, 3, 2), "4D AH 3S 2D 5C")]


    /* Pull the winner */
    let winner = hands
        .iter()
        .filter(|h| hands[0].eq(&h)) // ISSUE?
        .map(|h| h.1)
        .collect::<Vec<&'a str>>();
    Some(winner)
}


/* To-do */
// fixing 'test_aces_can_end_a_straight_low' FAILED
// fixing 'test_a_tie_has_multiple_winners' FAILED
// redesign Card
// refactor matching patterns in 'impl Hand'

/* Resources */
// lifetimes explainer
// https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html
// a borrow checker will verify the length of each scope in compile time
// the smaller lifetime will be still valid
//
// map()
// https://doc.rust-lang.org/rust-by-example/error/option_unwrap/map.html
// slice.windows()
// https://doc.rust-lang.org/std/primitive.slice.html#method.windows
// sort_unstable_by()
// https://doc.rust-lang.org/std/primitive.slice.html#method.sort_unstable_by
// as_slice()
// https://doc.rust-lang.org/std/vec/struct.Vec.html#method.as_slice
//
// Rust poker hand ranker (complex..)
// https://github.com/pieterdopheide/poker