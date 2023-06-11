impl Solution {
    pub fn is_fascinating(n: i32) -> bool {
        let n = n as i128;
        let n2 = 2 * n;
        let n3 = 3 * n;

        let mut q: Vec<i128> = vec![
            format!("{}{}{}", n, n2, n2).parse::<i128>().unwrap(),
            format!("{}{}{}", n, n2, n3).parse::<i128>().unwrap(),
            format!("{}{}{}", n, n3, n3).parse::<i128>().unwrap(),
            format!("{}{}{}", n, n3, n2).parse::<i128>().unwrap(),
        ];

        let mut ans = false;

        while let Some(mut num) = q.pop() {
            let mut have = vec![0; 10];
            while num > 0 {
                have[(num % 10) as usize] += 1;
                num /= 10;
            }
            if have.iter().filter(|x| x == &&1).count() == 9 && have[0] == 0 {
                ans = true;
                break;
            }
        }

        ans
    }
}
