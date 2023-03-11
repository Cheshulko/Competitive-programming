// https://www.codingame.com/ide/puzzle/tan-network

use std::cmp::Reverse;
use std::collections::{BTreeMap, BinaryHeap};
use std::io;
use std::ops::Add;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

struct Graph<V, E> {
    edges: BTreeMap<V, BTreeMap<V, E>>,
}

impl<V, E> Graph<V, E> {
    fn new() -> Self {
        Graph {
            edges: BTreeMap::new(),
        }
    }
}

impl<V, E> Graph<V, E>
where
    V: Ord + Copy,
    E: Ord + Copy + Add<Output = E>,
{
    fn dijkstra(&self, start: &V) -> BTreeMap<V, Option<(V, E)>> {
        let mut ans = BTreeMap::new();
        let mut priority_queue = BinaryHeap::new();

        ans.insert(*start, None);

        for (to, weight) in &self.edges[start] {
            ans.insert(*to, Some((*start, *weight)));
            priority_queue.push(Reverse((*weight, to, start)));
        }

        while let Some(Reverse((dist_to, to, prev))) = priority_queue.pop() {
            match ans[to] {
                Some((p, d)) if p == *prev && d == dist_to => {}
                _ => continue,
            }

            for (next, weight) in &self.edges[to] {
                match ans.get(next) {
                    Some(Some((_, dist_next))) if dist_to + *weight >= *dist_next => {}
                    Some(None) => {}
                    _ => {
                        ans.insert(*next, Some((*to, *weight + dist_to)));
                        priority_queue.push(Reverse((*weight + dist_to, next, to)));
                    }
                }
            }
        }

        ans
    }

    fn add_vertex(&mut self, v: V) {
        let _ = &self.edges.insert(v, BTreeMap::new());
    }

    fn add_edge(&mut self, v1: V, v2: V, c: E) {
        let _ = &self
            .edges
            .entry(v1)
            .or_insert_with(BTreeMap::new)
            .insert(v2, c);
    }
}

#[derive(Debug)]
struct Point {
    id: String,
    name: String,
    lon: f64,
    lat: f64,
}

impl Point {
    fn parse() -> Self {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();

        let parsed: Vec<&str> = input_line.trim_matches('\n').split(',').collect();

        Point {
            id: parsed[0].to_string(),
            name: parsed[1].trim_matches('\"').to_string(),
            lat: parse_input!(parsed[3], f64) * std::f64::consts::PI / 180.0,
            lon: parse_input!(parsed[4], f64) * std::f64::consts::PI / 180.0,
        }
    }

    fn distance(&self, to: &Point) -> i64 {
        let x = (to.lon - self.lon) * ((to.lat + self.lat) / 2.0).cos();
        let y = to.lat - self.lat;

        (6371.0 * (x * x + y * y).sqrt() * 1_000_000.0).round() as i64
    }
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let start_point = input_line.trim().to_string();

    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let end_point = input_line.trim().to_string();

    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, usize);

    let mut graph: Graph<usize, i64> = Graph::new();
    let mut points: Vec<Point> = vec![];
    let mut name_to_index: BTreeMap<String, usize> = BTreeMap::new();

    (0..n as usize).for_each(|i| {
        let point = Point::parse();
        name_to_index.insert(point.id.clone(), i);
        points.push(point);
        graph.add_vertex(i)
    });

    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let m = parse_input!(input_line, usize);
    (0..m).for_each(|_| {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();

        let parsed: Vec<&str> = input_line.trim_matches('\n').split(' ').collect();
        let from = parsed.iter().nth(0).unwrap();
        let to = parsed.iter().nth(1).unwrap();

        let index_0 = name_to_index.get(*from).unwrap();
        let index_1 = name_to_index.get(*to).unwrap();

        let point_0: &Point = points.iter().nth(*index_0).unwrap();
        let point_1: &Point = points.iter().nth(*index_1).unwrap();
        graph.add_edge(*index_0, *index_1, point_0.distance(point_1));
    });

    let start_point_index = *name_to_index.get(&start_point).unwrap();

    let mut end_point_index = *name_to_index.get(&end_point).unwrap();
    let ans = graph.dijkstra(&start_point_index);

    if let None = ans.get(&end_point_index) {
        println!("IMPOSSIBLE");
    } else {
        let mut path: Vec<String> = vec![];

        path.push(points.iter().nth(end_point_index).unwrap().name.to_string());

        while let Some(x) = ans.get(&end_point_index) {
            match x {
                Some((point, _)) => {
                    path.push(points.iter().nth(*point).unwrap().name.to_string());
                    end_point_index = *point;
                }
                None => break,
            }
        }

        path.iter().rev().for_each(|point| println!("{}", point));
    }
}
