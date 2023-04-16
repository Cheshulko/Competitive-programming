pub fn is_armstrong_number(num: u32) -> bool {
    let numi = num as u128;
    let p = (num as f64).log10().floor() as u32 + 1;
    let mut r: u128 = 0;
    let mut numm = num;
    while numm > 0 {
        r += (numm % 10).pow(p) as u128;
        numm /= 10;
    }
    numi == r
}
