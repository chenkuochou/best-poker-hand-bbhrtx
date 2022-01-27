// Cards combination eg. 4D
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

// Hands of poker patterns
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
        println!("sorted:{:?}", cards); // [Card(5, 'H'), Card(5, 'D'), Card(4, 'S'), Card(4, 'H'), Card(4, 'D')]
        
        // straight & flush vars
        // let straight = cards.windows(); //i[0].0 == i[1].0+1
        // let flush = cards.windows(); // i[0]1 == i[1].1

        // matching straight & flush combinations
        // straight + flush = StraightFlush
        // !(straight || flush) + 4 same cards = FourOfAKind
        // !(straight || flush) + 2 same card pair = TwoPair

        // need a parttern for poker patterns, counted and sorted
        // (count, number)
        // eg. full house: [(3, 5), (2, 9)] from ["5H 5S 5D 9S 9D", "5H 5S 5D 8S 8D"]
        FullHouse(4,7)
    }
}

pub fn winning_hands<'a>(hands: &[&'a str]) -> Option<Vec<&'a str>> {
    unimplemented!("Out of {:?}, which hand wins?", hands)
    
    // hands.iter() 
}



/* Resources */
// lifetimes explainer
// https://doc.rust-lang.org/reference/lifetime-elision.html
// https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html
//
// map
// https://doc.rust-lang.org/rust-by-example/error/option_unwrap/map.html?highlight=.map#combinators-map
//
// sort_unstable_by 
// https://doc.rust-lang.org/std/primitive.slice.html#method.sort_unstable_by
