// https://leetcode.com/problems/course-schedule

struct Solution {}

impl Solution {
    fn top_sort(cur: usize, graph: &Vec<Vec<usize>>, used: &mut Vec<bool>, v: &mut Vec<usize>) {
        used[cur] = true;

        for to in &graph[cur] {
            let to = *to;
            if !used[to] {
                Solution::top_sort(to, graph, used, v);
            }
        }

        v.push(cur);
    }

    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let num_courses = num_courses as usize;
        let mut used = vec![false; num_courses];

        let mut graph = vec![vec![]; num_courses];

        for p in prerequisites.into_iter() {
            graph[p[1] as usize].push(p[0] as usize);
        }

        let mut v = vec![];

        for i in 0..num_courses {
            if !used[i] {
                Solution::top_sort(i, &graph, &mut used, &mut v);
            }
        }

        let v = v.into_iter().enumerate().collect::<Vec<_>>();

        let mut can = true;

        let mut indx = vec![-1; num_courses];
        for u in &v {
            indx[u.1] = u.0 as i32;
        }

        for u in v.iter().rev() {
            can &= !graph[u.1].iter().any(|a| indx[*a] >= indx[u.1]);
        }

        can
    }
}
