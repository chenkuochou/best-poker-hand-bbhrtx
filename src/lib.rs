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
        // [Card(5, 'H'), Card(5, 'D'), Card(4, 'S'), Card(4, 'H'), Card(4, 'D')]


        /* Sorted parttern (number, count) for each card */
        let mut number_count = (1..=14)
            .map(|n| (n, cards.iter().filter(|i| i.0 == n).count() as u8))
            .filter(|(_, count)| *count != 0)
            .collect::<Vec<_>>();
        // println!("number_count:{:?}", number_count);


        number_count.sort_by(|x, y| y.1.cmp(&x.1).then(y.0.cmp(&x.0)));
        // I asked for the above structure 
        // https://stackoverflow.com/questions/70919627/how-to-sort-items-inside-a-vector-in-rust


        let nc = number_count.as_slice();
        // println!("nc:{:?}", nc);
        // [(4, 3), (8, 1), (5, 1)] from {"4S 5H 4C 8D 4H"}


        /* Match patterns & return rank(s) */
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
            }),
            (_, _, 3, _) => ThreeOfAKind(nc[0].0, nc[1].0, nc[2].0),
            (_, _, 2, 2) => TwoPair(nc[0].0, nc[1].0, nc[2].0),
            (_, _, 2, _) => OnePair(nc[0].0, nc[1].0, nc[2].0, nc[3].0),
            _ => HighCard(cards[0].0, cards[1].0, cards[2].0, cards[3].0, cards[4].0),
        }
    }
}

pub fn winning_hands<'a>(hands: &[&'a str]) -> Option<Vec<&'a str>> {
    // unimplemented!("Out of {:?}, which hand wins?", hands)

    // The returned referecne 'Option<>' will also be valid for the length of 'hands'
    // CANNOT return a referecne to somethig created inside the fucntion!

    // hands.iter()
    // compare pattern then rank
    

}

/* Resources */
// lifetimes explainer
// https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html
// a borrow checker will verify the length of each scope in compile time
// the smaller lifetime will be still valid"
//
// map()
// https://doc.rust-lang.org/rust-by-example/error/option_unwrap/map.html?highlight=.map#combinators-map
// sort_unstable_by()
// https://doc.rust-lang.org/std/primitive.slice.html#method.sort_unstable_by
// slice.windows()
// https://doc.rust-lang.org/std/primitive.slice.html#method.windows
// as_slice()
// https://doc.rust-lang.org/std/vec/struct.Vec.html#method.as_slice
// as_mut_slice()
// https://doc.rust-lang.org/std/vec/struct.Vec.html#method.as_mut_slice
//
// Rust poker hand ranker (complex..)
// https://github.com/pieterdopheide/poker