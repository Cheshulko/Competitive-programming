// https://www.codingame.com/ide/puzzle/network-cabling
use std::io;

/*
3
0 5
1 1
2 10

9
0 0
100 2
0 3
1 0
5 2
1 3
9 2
2 3
11 0
*/

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

fn calc(index: usize, grid: &Vec<(i64, i64)>) -> i64 {
    let mut result = 0;
    let value = grid.get(index).unwrap().1;
    for (_, y) in grid.iter() {
        result += (y - value).abs();
    }
    result
}
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, i64);

    let mut grid: Vec<(i64, i64)> = vec![];

    (0..n).for_each(|_| {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let x = parse_input!(inputs[0], i64);
        let y = parse_input!(inputs[1], i64);
        grid.push((x, y));
    });

    let right = grid.iter().max_by(|x, y| x.0.cmp(&y.0)).unwrap().0;
    let left = grid.iter().min_by(|x, y| x.0.cmp(&y.0)).unwrap().0;

    let x_length = (right - left).abs();

    let mut low: usize = 0;
    let mut up = grid.len() - 1;

    grid.sort_by(|a, b| a.1.cmp(&b.1));

    let calc_low_up = |low: usize, up: usize| (calc(low, &grid), calc(up, &grid));

    let (mut calc_low, mut calc_up) = calc_low_up(low, up);

    while up - low > 1 {
        let mid = (low + up) / 2;
        let calc_mid = calc(mid, &grid);

        if calc_low > calc_up {
            calc_low = calc_mid;
            low = mid;
        } else {
            calc_up = calc_mid;
            up = mid;
        }
    }

    let mut result_index = low;
    if calc_low != calc_up {
        let (calc_low, calc_up) = calc_low_up(low, up);

        if calc_low > calc_up {
            result_index = up;
        }
    }

    println!("{}", calc(result_index, &grid) + x_length);
}
