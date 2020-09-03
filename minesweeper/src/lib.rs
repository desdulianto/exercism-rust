fn count_mines(i: usize, j: usize, minefield: &[&str]) -> usize {
    let (x, y) = (i as isize, j as isize);
    [
        (x - 1, y - 1),
        (x - 1, y),
        (x - 1, y + 1),
        (x, y - 1),
        (x, y + 1),
        (x + 1, y - 1),
        (x + 1, y),
        (x + 1, y + 1),
    ]
    .iter()
    .map(|&(x, y)| {
        if Some('*')
            == minefield
                .get(x as usize)
                .or(Some(&""))
                .unwrap()
                .chars()
                .nth(y as usize)
        {
            1
        } else {
            0
        }
    })
    .sum()
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    minefield
        .iter()
        .enumerate()
        .map(|r| {
            let (i, row) = r;
            row.chars()
                .enumerate()
                .map(|c| match c {
                    (_, '*') => String::from("*"),
                    (col, _) => {
                        let mines = count_mines(i, col, &minefield);
                        if mines > 0 {
                            mines.to_string()
                        } else {
                            String::from(" ")
                        }
                    }
                })
                .collect::<String>()
        })
        .collect::<Vec<String>>()
}
