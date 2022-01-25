#[derive(Debug, Ord, Eq, PartialEq, PartialOrd)]
enum Hand {}
enum Card {}
enum Sorted {}

pub fn winning_hands<'a>(hands: &[&'a str]) -> Option<Vec<&'a str>> {
    unimplemented!("Out of {:?}, which hand wins?", hands)
}
