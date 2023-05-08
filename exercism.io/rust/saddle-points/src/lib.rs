pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let rows = input.iter().map(|row| row.iter().max()).collect::<Vec<_>>();

    let num_column = input.first().unwrap_or(&vec![]).len();
    let columns = (0..num_column)
        .map(|column| input.iter().map(|row| &row[column]).min())
        .collect::<Vec<_>>();

    let mut ans = vec![];

    for (i, v) in rows.iter().enumerate() {
        for (j, u) in columns.iter().enumerate() {
            if v == u {
                ans.push((i, j));
            }
        }
    }

    ans
}
