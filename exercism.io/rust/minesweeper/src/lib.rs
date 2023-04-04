pub fn annotate(minefield: &[&str]) -> Vec<String> {
    fn check(i: usize, j: usize, minefield: &[&str]) -> bool {
        minefield
            .get(i as usize)
            .and_then(|row| {
                row.as_bytes()
                    .get(j as usize)
                    .and_then(|c| Some(*c == b'*'))
            })
            .unwrap_or(false)
    }

    (0..minefield.len())
        .map(|i| {
            (0..minefield[i].len())
                .map(|j| {
                    if minefield[i].as_bytes()[j] == b'*' {
                        '*'
                    } else {
                        match [
                            (-1, 0),
                            (-1, 1),
                            (0, 1),
                            (1, 1),
                            (1, 0),
                            (1, -1),
                            (0, -1),
                            (-1, -1),
                        ]
                        .iter()
                        .map(|(di, dj)| (i as i32 + di, j as i32 + dj))
                        .filter(|(to_i, to_j)| to_i >= &0 && to_j >= &0)
                        .map(|(to_i, to_j)| (to_i as usize, to_j as usize))
                        .fold(0, |s, x| s + if check(x.0, x.1, minefield) { 1 } else { 0 })
                        {
                            0 => ' ',
                            x @ _ => x.to_string().chars().last().unwrap(),
                        }
                    }
                })
                .collect::<String>()
        })
        .collect::<Vec<String>>()
}
