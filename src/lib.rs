#[derive(Debug, Ord, Eq, PartialOrd, PartialEq)]
// Found cards eg. 4S
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

#[derive(Debug, Ord, Eq, PartialOrd, PartialEq)]
// Found hands of poker patterns
enum Hand {}
impl Hand {
    // sorting
    // find straight & flush

    // matching
}

pub fn winning_hands<'a>(hands: &[&'a str]) -> Option<Vec<&'a str>> {
    unimplemented!("Out of {:?}, which hand wins?", hands)
}
