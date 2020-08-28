use std::cmp::Ordering;

pub fn verse(n: u32) -> String {
    match n.cmp(&1) {
        Ordering::Less => format!("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n"),
        Ordering::Equal => format!("1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n"),
        Ordering::Greater => format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} {} of beer on the wall.\n", n, n, n-1, if n-1 > 1 { "bottles" } else { "bottle"} )
    }
}

pub fn sing(start: u32, end: u32) -> String {
    (end..=start)
        .rev()
        .map(|x| verse(x))
        .collect::<Vec<String>>()
        .join("\n")
}
