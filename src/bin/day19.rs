use regex::Regex;
use std::cmp::max;
use std::collections::HashSet;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
struct Blueprint {
    robots_costs: Vec<(u16, u16)>,
}

fn simulate(blueprint: &Blueprint, time_limit: i32) -> u16 {
    let mut queue = VecDeque::new();
    let mut cache = HashSet::new();

    let max_ore_robots = *blueprint
        .robots_costs
        .iter()
        .map(|(ore, _)| ore)
        .max()
        .unwrap();
    let max_clay_robots = blueprint.robots_costs[2].1;
    let max_obsidian_robots = blueprint.robots_costs[3].1;

    //    resources, robots
    // State: (time, (ore,clay,obsidian, geode), (ore,clay,obsidian,geode))
    queue.push_front((0, (0, 0, 0, 0), (1, 0, 0, 0)));
    let mut max_geodes = 0;
    while !queue.is_empty() {
        let (
            time,
            (ore, clay, obsidian, geode),
            (robots_ore, robots_clay, robots_obsidian, robots_geode),
        ) = queue.pop_back().unwrap();
        if time == time_limit {
            max_geodes = max(max_geodes, geode);
            continue;
        }
        // heuristics to limit number of states
        if time > 16 && robots_obsidian == 0 {
            continue;
        }
        if time > 27 && robots_geode == 0 {
            continue;
        }

        let k = (
            time,
            (ore, clay, obsidian, geode),
            (robots_ore, robots_clay, robots_obsidian, robots_geode),
        );
        if cache.contains(&k) {
            continue;
        } else {
            cache.insert(k);
        }

        // if we can build geode robot, we do it
        if obsidian >= blueprint.robots_costs[3].1 && ore >= blueprint.robots_costs[3].0 {
            queue.push_back((
                time + 1,
                (
                    ore - blueprint.robots_costs[3].0 + robots_ore,
                    clay + robots_clay,
                    obsidian - blueprint.robots_costs[3].1 + robots_obsidian,
                    geode + robots_geode,
                ),
                (robots_ore, robots_clay, robots_obsidian, robots_geode + 1),
            ));
        }
        // if we can build obsidian robot, we do it
        else {
            if robots_obsidian < max_obsidian_robots
                && clay >= blueprint.robots_costs[2].1
                && ore >= blueprint.robots_costs[2].0
            {
                queue.push_back((
                    time + 1,
                    (
                        ore - blueprint.robots_costs[2].0 + robots_ore,
                        clay - blueprint.robots_costs[2].1 + robots_clay,
                        obsidian + robots_obsidian,
                        geode + robots_geode,
                    ),
                    (robots_ore, robots_clay, robots_obsidian + 1, robots_geode),
                ));
            }
            // build clay robot
            if robots_clay < max_clay_robots && ore >= blueprint.robots_costs[1].0 {
                queue.push_back((
                    time + 1,
                    (
                        ore - blueprint.robots_costs[1].0 + robots_ore,
                        clay + robots_clay,
                        obsidian + robots_obsidian,
                        geode + robots_geode,
                    ),
                    (robots_ore, robots_clay + 1, robots_obsidian, robots_geode),
                ));
            }

            // build ore robot
            if robots_ore < max_ore_robots && ore >= blueprint.robots_costs[0].0 {
                queue.push_back((
                    time + 1,
                    (
                        ore - blueprint.robots_costs[0].0 + robots_ore,
                        clay + robots_clay,
                        obsidian + robots_obsidian,
                        geode + robots_geode,
                    ),
                    (robots_ore + 1, robots_clay, robots_obsidian, robots_geode),
                ));
            }

            // no robot built
            queue.push_back((
                time + 1,
                (
                    ore + robots_ore,
                    clay + robots_clay,
                    obsidian + robots_obsidian,
                    geode + robots_geode,
                ),
                (robots_ore, robots_clay, robots_obsidian, robots_geode),
            ));
        }
    }
    max_geodes
}

fn parse_input() -> Vec<Blueprint> {
    let re = Regex::new(r"Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.$").unwrap();
    include_str!("../../input/day19")
        .lines()
        .map(|line| {
            let cap = re.captures_iter(line).next().unwrap();
            let costs = vec![
                (cap[2].parse::<u16>().unwrap(), 0),
                (cap[3].parse::<u16>().unwrap(), 0),
                (
                    cap[4].parse::<u16>().unwrap(),
                    cap[5].parse::<u16>().unwrap(),
                ),
                (
                    cap[6].parse::<u16>().unwrap(),
                    cap[7].parse::<u16>().unwrap(),
                ),
            ];
            Blueprint {
                robots_costs: costs,
            }
        })
        .collect::<Vec<_>>()
}

fn day19_1() {
    let blueprints = parse_input();
    let max_values = blueprints
        .iter()
        .map(|blueprint| simulate(blueprint, 24))
        .collect::<Vec<_>>();

    let solution = max_values
        .iter()
        .enumerate()
        .fold(0, |acc, (idx, &val)| acc + (idx + 1) * val as usize);

    println!("p1: {solution:?}");
}

fn day19_2() {
    let blueprints = parse_input();
    let max_values = blueprints
        .iter()
        .take(3)
        .map(|blueprint| simulate(blueprint, 32))
        .collect::<Vec<_>>();

    let solution = max_values.iter().fold(1, |acc, &val| acc * val as usize);
    println!("p2: {solution:?}");
}

fn main() {
    day19_1();
    day19_2();
}
