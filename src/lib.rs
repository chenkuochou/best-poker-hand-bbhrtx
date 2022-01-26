#[derive(Debug, Ord, Eq, PartialOrd, PartialEq)]
struct Card(u8, char);
impl Card {}

#[derive(Debug, Ord, Eq, PartialOrd, PartialEq)]
enum Hand {}
impl Hand {
    // sorting
    // find straight & flush
}



pub fn winning_hands<'a>(hands: &[&'a str]) -> Option<Vec<&'a str>> {
    unimplemented!("Out of {:?}, which hand wins?", hands)
}
