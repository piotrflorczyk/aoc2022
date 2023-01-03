use regex::Regex;
use std::cmp::{max, min};
use std::collections::hash_map::Entry::Vacant;
use std::collections::VecDeque;
use std::{collections::HashMap, collections::HashSet, hash::Hash};

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Valve {
    index: usize,
    flow: u32,
    neighbours: Vec<String>,
}

fn parse() -> HashMap<String, Valve> {
    let re =
        Regex::new(r"Valve (\w+) has flow rate=([^;]+); tunnels? leads? to valves? (.+)").unwrap();
    include_str!("../../input/day16")
        .lines()
        .enumerate()
        .map(|(idx, line)| {
            let cap = re.captures_iter(line).next().unwrap();
            (
                cap[1].to_string(),
                Valve {
                    index: idx,
                    flow: cap[2].parse::<u32>().unwrap(),
                    neighbours: cap[3]
                        .split(", ")
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>(),
                },
            )
        })
        .collect::<HashMap<_, _>>()
}

// Floyd-Warshall
fn find_distances(valves: &HashMap<String, Valve>) -> Vec<Vec<u32>> {
    let mut dist = vec![vec![1000; valves.len()]; valves.len()];
    valves.iter().for_each(|(_, v)| {
        dist[v.index][v.index] = 0;
        v.neighbours.iter().for_each(|v2| {
            dist[v.index][valves[v2].index] = 1;
        });
    });

    for i in 0..dist.len() {
        for j in 0..dist.len() {
            for k in 0..dist.len() {
                dist[j][k] = min(dist[j][k], dist[j][i] + dist[i][k]);
            }
        }
    }
    dist
}

fn opened_to_bitset(
    valves: &HashMap<String, Valve>,
    non_zero_valves: &HashSet<String>,
    remaining: &HashSet<String>,
) -> u64 {
    non_zero_valves
        .iter()
        .filter(|&v| !remaining.contains(v))
        .fold(0, |acc, v| acc | (1 << valves[v].index))
}

fn find_max_released_pressure(
    valves: &HashMap<String, Valve>,
    distances: &[Vec<u32>],
    non_zero_valves: &HashSet<String>,
    time_limit: i32,
) -> HashMap<u64, u32> {
    let mut queue = VecDeque::new();
    let mut max_pressures = HashMap::new();
    queue.push_back(("AA".to_string(), time_limit, 0, non_zero_valves.clone()));
    while let Some((curr, time_left, pressure, remaining)) = queue.pop_back() {
        let state = opened_to_bitset(valves, non_zero_valves, &remaining);
        if let Vacant(e) = max_pressures.entry(state) {
            e.insert(pressure);
        } else {
            let curr_max: u32 = max_pressures[&state];
            *max_pressures.get_mut(&state).unwrap() = max(curr_max, pressure);
        }

        for to_open in &remaining {
            let distance = distances[valves[&curr].index][valves[to_open].index] as i32;
            let new_time = time_left - distance - 1;
            if new_time >= 0 {
                let released_pressure = new_time as u32 * valves[to_open].flow;
                let mut new_remaining = remaining.clone();
                new_remaining.remove(to_open);
                queue.push_back((
                    to_open.clone(),
                    new_time,
                    pressure + released_pressure,
                    new_remaining,
                ));
            }
        }
    }
    max_pressures
}

fn dp_with_array(
    valves: &HashMap<String, Valve>,
    distances: &[Vec<u32>],
    time_limit: usize,
) -> u32 {
    let mut non_zero_valves = valves
        .iter()
        .filter(|(_, v)| v.flow > 0)
        .map(|(_, v)| v)
        .collect::<Vec<_>>();
    non_zero_valves.insert(0, &valves["AA"]);
    let max_set = 1 << non_zero_valves.len();

    let mut dp_arr = vec![vec![vec![0; max_set]; non_zero_valves.len()]; time_limit + 1];

    for time_left in 0..time_limit + 1 {
        for curr_valve in 0..non_zero_valves.len() {
            for remining_set in 0..max_set {
                let mut curr_best = dp_arr[time_left][curr_valve][remining_set];
                for i in 0..non_zero_valves.len() {
                    if (remining_set & (1 << i)) == (1 << i) {
                        let distance = distances[non_zero_valves[curr_valve].index]
                            [non_zero_valves[i].index]
                            + 1;
                        if distance <= time_left as u32 {
                            curr_best = max(
                                curr_best,
                                non_zero_valves[curr_valve].flow * time_left as u32
                                    + dp_arr[time_left - distance as usize][i]
                                        [remining_set - (1 << i)],
                            )
                        }
                    }
                }
                dp_arr[time_left][curr_valve][remining_set] = curr_best;
            }
        }
    }
    dp_arr[time_limit][0][max_set - 1]
}

fn main() {
    let valves = parse();
    let distances = find_distances(&valves);

    let non_zero_valves = valves
        .iter()
        .filter(|(_, v)| v.flow != 0)
        .map(|(name, _)| name.clone())
        .collect::<HashSet<_>>();

    let max_released = find_max_released_pressure(&valves, &distances, &non_zero_valves, 30);
    let p1_max = *max_released.values().max().unwrap();
    println!("max pressure p1: {p1_max:?}");

    let max_pressures = find_max_released_pressure(&valves, &distances, &non_zero_valves, 26);

    let mut p2_max = 0;
    for set_1 in max_pressures.keys() {
        for set_2 in max_pressures.keys() {
            // sets are disjoint, so both elephant and elf took different routes
            if set_1 & set_2 == 0 {
                p2_max = max(p2_max, max_pressures[set_1] + max_pressures[set_2]);
            }
        }
    }
    println!("max pressure p2: {p2_max:?}");

    let dp_p1 = dp_with_array(&valves, &distances, 30);
    println!("dp with array p1: {dp_p1:?}");
}
