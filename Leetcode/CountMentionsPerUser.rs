// https://leetcode.com/problems/count-mentions-per-user

struct Solution {}

impl Solution {
    pub fn count_mentions(number_of_users: i32, mut events: Vec<Vec<String>>) -> Vec<i32> {
        events.sort_unstable_by(|e1, e2| {
            let t1 = e1[1].parse::<i32>().unwrap();
            let t2 = e2[1].parse::<i32>().unwrap();

            match t1.cmp(&t2) {
                std::cmp::Ordering::Equal => e2[0].cmp(&e1[0]),
                e => e,
            }
        });

        let number_of_users = number_of_users as usize;

        let mut user_state = vec![-1; number_of_users];

        let mut ans = vec![0; number_of_users];
        for event in events {
            match event[0].as_str() {
                "MESSAGE" => match event[2].as_str() {
                    "ALL" => {
                        for i in 0..number_of_users {
                            ans[i] += 1;
                        }
                    }
                    "HERE" => {
                        let time = event[1].parse::<i32>().unwrap();
                        for i in 0..number_of_users {
                            if user_state[i] <= time {
                                ans[i] += 1;
                            }
                        }
                    }
                    list => {
                        for id in list.split(" ") {
                            let id = (id[2..]).parse::<usize>().unwrap();
                            ans[id] += 1;
                        }
                    }
                },
                "OFFLINE" => {
                    let time = event[1].parse::<i32>().unwrap();
                    let id = event[2].parse::<usize>().unwrap();

                    user_state[id] = time + 60;
                }
                _ => unreachable!(),
            }
        }

        ans
    }
}
