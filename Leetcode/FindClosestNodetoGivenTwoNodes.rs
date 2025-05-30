impl Solution {
    pub fn closest_meeting_node(edges: Vec<i32>, node1: i32, node2: i32) -> i32 {
        let n = edges.len();

        let mut times1 = vec![None; n];
        let mut times2 = vec![None; n];

        fn dfs(cur: usize, time: i32, edges: &Vec<i32>, times: &mut Vec<Option<i32>>) {
            times[cur] = Some(time);

            if edges[cur] != -1 && times[edges[cur] as usize].is_none() {
                dfs(edges[cur] as usize, time + 1, edges, times);
            }
        }

        dfs(node1 as usize, 0, &edges, &mut times1);
        dfs(node2 as usize, 0, &edges, &mut times2);

        (0..n)
            .filter_map(|node| {
                times1[node]
                    .and_then(|x| times2[node].map(|y| x.max(y)))
                    .and_then(|mi| Some((mi, node as i32)))
            })
            .fold((i32::MAX, -1), |cur, next| cur.min(next))
            .1
    }
}
