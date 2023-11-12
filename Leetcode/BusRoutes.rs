// https://leetcode.com/problems/bus-routes

use std::collections::{HashMap, VecDeque};

impl Solution {
    pub fn num_buses_to_destination(routes: Vec<Vec<i32>>, source: i32, target: i32) -> i32 {
        if source == target {
            return 0;
        }

        let n = routes.len();

        let mut buses_at_station = HashMap::<i32, Vec<usize>>::new();
        let mut connected = vec![vec![false; n]; n];

        for (route_bus, route) in routes.iter().enumerate() {
            for station in route {
                let buses = buses_at_station.entry(*station).or_default();

                for bus in buses.iter() {
                    connected[*bus][route_bus] = true;
                    connected[route_bus][*bus] = true;
                }

                buses.push(route_bus);
            }
        }

        let mut q = VecDeque::<(i32, usize)>::new();
        let mut dist = vec![None; n];

        if let Some(buses_at_source) = buses_at_station.get(&source) {
            for bus_at_source in buses_at_source {
                q.push_back((1, *bus_at_source));
                dist[*bus_at_source] = Some(1);
            }
        }

        while let Some((cnt, cur)) = q.pop_back() {
            for to in 0..n {
                if connected[cur][to] {
                    if let Some(dist_to) = dist[to] {
                        if dist_to > cnt + 1 {
                            dist[to] = Some(cnt + 1);
                            q.push_back((cnt + 1, to));
                        }
                    } else {
                        dist[to] = Some(cnt + 1);
                        q.push_back((cnt + 1, to));
                    }
                }
            }
        }

        if let Some(buses_at_target) = buses_at_station.get(&target) {
            buses_at_target
                .iter()
                .map(|bus_at_target| dist[*bus_at_target])
                .filter_map(|bus_at_target| bus_at_target)
                .min()
                .unwrap_or(-1)
        } else {
            -1
        }
    }
}
