// https://leetcode.com/problems/maximize-the-minimum-powered-city

struct Solution {}

impl Solution {
    pub fn max_power(stations: Vec<i32>, r: i32, k: i32) -> i64 {
        let r = r as usize;
        let k = k as i64;
        let stations = stations.into_iter().map(|s| s as i64).collect::<Vec<_>>();

        fn can(r: usize, mut k: i64, target: i64, stations: &Vec<i64>) -> bool {
            let n = stations.len();
            let mut pref = vec![0; n + 1];

            // r = 2
            // 0 1 2 3 4 5 6 ... n - 1
            // [   ^   ]
            // 0 1 2 3 4 5 6
            //     [   ^   ]
            for i in r..n {
                pref[i + 1] = pref[i];

                let j = i - r;
                let j_left = if j >= r { j - r } else { 0 };
                let added = pref[i + 1] - pref[j_left];
                if stations[j] + added < target {
                    let need = target - (stations[j] + added);
                    if need > k {
                        return false;
                    }
                    k -= need;
                    pref[i + 1] += need;
                }
            }

            pref[n] += k;

            let i_left = if n > r { n - r - 1 } else { 0 };
            let added = pref[n] - pref[i_left];
            for i in (n - n.min(r))..n {
                if stations[i] + added < target {
                    return false;
                }
            }

            return true;
        }

        let n = stations.len();
        let mut stations_pref = vec![0; n + 1];
        for i in 0..n {
            stations_pref[i + 1] += stations_pref[i] + stations[i];
        }

        let mut stations_sum = vec![0; n];
        for i in 0..n {
            let i_left = if i >= r { i - r } else { 0 };
            let i_right = (i + r + 1).min(n);

            stations_sum[i] = stations_pref[i_right] - stations_pref[i_left];
        }

        let mut le = stations_sum.iter().min().cloned().unwrap();
        let mut ri = stations_sum.iter().max().cloned().unwrap() + k + 1;

        while ri - le > 1 {
            let m = (ri + le) >> 1;
            if can(r, k, m, &stations_sum) {
                le = m;
            } else {
                ri = m;
            }
        }

        le
    }
}
