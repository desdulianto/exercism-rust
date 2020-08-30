fn count_mines(i: usize, j: usize, minefield: &[&str]) -> usize {
    let (rows, cols) = (
        minefield.len(),
        if !minefield[i].is_empty() {
            minefield[0].len()
        } else {
            0
        },
    );
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
    .filter(|&&(x, y)| x >= 0 && x < (rows as isize) && y >= 0 && y < (cols as isize))
    .map(|&(x, y)| {
        let (x, y) = (x as usize, y as usize);
        if Some('*') == minefield[x].chars().nth(y) {
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
