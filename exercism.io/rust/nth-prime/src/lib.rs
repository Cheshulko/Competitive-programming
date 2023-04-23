const MAX: usize = 1_000_000;

pub fn nth(n: usize) -> usize {
    let mut pr = vec![true; MAX + 1];
    pr[0] = false;
    pr[1] = false;

    for i in 2..=MAX {
        if pr[i] {
            for j in (i + i..=MAX).step_by(i) {
                pr[j] = false;
            }
        }
    }

    pr.iter()
        .enumerate()
        .filter(|x| *x.1)
        .map(|x| x.0)
        .nth(n)
        .unwrap()
}
