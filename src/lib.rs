// Read cards e.g. Card(5, 'H')
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

// Hands of poker patterns (rules to compare)
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

        // sorting
        cards.sort_by(|x, y| y.cmp(x));
        // println!("sorted:{:?}", cards);
        // [Card(5, 'H'), Card(5, 'D'), Card(4, 'S'), Card(4, 'H'), Card(4, 'D')]

        // a sorted parttern (count, number) for each card
        let mut count_number = (1..=14)
            .map(|n| (cards.iter().filter(|i| i.0 == n).count(), n as u8))
            .filter(|(n, _)| *n != 0)
            .collect::<Vec<_>>();
        count_number.sort_by(|x, y| y.cmp(x));
        let cn = count_number.as_slice();
        // println!("cn:{:?}", cn);
        // [(3, 4), (2, 9)] from [Card(9, 'S'), Card(9, 'D'), Card(4, 'S'), Card(4, 'H'), Card(4, 'D')]

        // match patterns & return rank(s)
        let straight = cards.windows(2).all(|w| w[0].0 == w[1].0 + 1);
        let flush = cards.windows(2).all(|w| w[0].1 == w[1].1);

        match (straight, flush, cn[0].0, cn[1].0) {
            (true, true, _, _) => StraightFlush(cards[4].0),
            (_, _, 4, _) => FourOfAKind(cn[0].1, cn[1].1),
            (_, _, 3, 2) => FullHouse(cn[0].1, cn[1].1),
            (_, true, _, _) => Flush(cards[0].0, cards[1].0, cards[2].0, cards[3].0, cards[4].0),
            (true, _, _, _) => Straight(match cards[0].0 {
                14 => 5,
                c => c,
            }),
            (_, _, 3, _) => ThreeOfAKind(cn[0].1, cn[1].1, cn[2].1),
            (_, _, 2, 2) => TwoPair(cn[0].1, cn[1].1, cn[2].1),
            (_, _, 2, _) => OnePair(cn[0].1, cn[1].1, cn[2].1, cn[3].1),
            _ => HighCard(cards[0].0, cards[1].0, cards[2].0, cards[3].0, cards[4].0),
        }
    }
}

pub fn winning_hands<'a>(hands: &[&'a str]) -> Option<Vec<&'a str>> {
    unimplemented!("Out of {:?}, which hand wins?", hands)

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
// map
// https://doc.rust-lang.org/rust-by-example/error/option_unwrap/map.html?highlight=.map#combinators-map
//
// sort_unstable_by
// https://doc.rust-lang.org/std/primitive.slice.html#method.sort_unstable_by
//
// slice.windows()
// https://doc.rust-lang.org/std/primitive.slice.html#method.windows
//
// as_slice()
// https://doc.rust-lang.org/std/vec/struct.Vec.html#method.as_slice
