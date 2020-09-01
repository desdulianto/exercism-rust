use pascals_triangle::PascalsTriangle;

fn main() {
    println!("hello");
    for row in 0..=5 {
        println!("==={}", row);
        PascalsTriangle::new(row).rows();
    }
}