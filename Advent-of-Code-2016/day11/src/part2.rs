use regex::Regex;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::hash::{Hash, Hasher};
use itertools::Itertools;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum Tech {
    RTG(String),
    MICROCHIP(String),
    NONE
}

impl PartialOrd for Tech {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Tech {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Tech::NONE, Tech::NONE) => Ordering::Equal,
            (Tech::NONE, _) => Ordering::Greater,
            (_, Tech::NONE) => Ordering::Less,
            (Tech::RTG(a), Tech::RTG(b)) => a.cmp(b),
            (Tech::RTG(_), Tech::MICROCHIP(_)) => Ordering::Less,
            (Tech::MICROCHIP(a), Tech::MICROCHIP(b)) => a.cmp(b),
            (Tech::MICROCHIP(_), Tech::RTG(_)) => Ordering::Greater,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct State {
    sorted_floors: Vec<Vec<Tech>>,
    elevator: usize,
    cost: i64, 
}

impl State {
    fn heuristic(&self) -> i64 {
        let mut h = 0;
        let n = self.sorted_floors.len();
        for i in 0..n {
            let nf = self.sorted_floors[i].len();
            h += (nf * (n - i - 1)) as i64;
        }
        h += 1 * self.cost;
        h
    }

    fn sort_floors(&mut self) {
        for floor in self.sorted_floors.iter_mut() {
            floor.sort();
        }
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // other.heuristic().cmp(&self.heuristic())
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn is_state_valid(state: &State) -> bool {
    for floor in state.sorted_floors.iter() {
        if !is_valid_floor(floor) {
            return false;
        }
    }
    return true
}

fn is_valid_floor(floor: &Vec<Tech>) -> bool {
    for tech in floor.iter() {
        match tech {
            Tech::RTG(_) => {},
            Tech::NONE => {}
            Tech::MICROCHIP(name_m) => {
                let mut has_compatible = false;
                let mut has_incompatible = false;

                for tech in floor.iter() {
                    match tech {
                        Tech::RTG(name_g) => {
                            if name_g == name_m {
                                has_compatible = true;
                            } else {
                                has_incompatible = true;
                            }
                        },
                        _ => {}
                    }
                }

                // println!("{} - {}", has_incompatible, has_compatible);
                if has_incompatible && !has_compatible {
                    return false
                }

            },
        }
    }

    return true;
}

fn elevate_tech(state: &State, techs: Vec<Tech>, from: usize, to: usize) -> State {

    assert!(techs.iter().any(|x| *x != Tech::NONE));
    let mut state = state.clone();

    for tech in techs {
        if tech == Tech::NONE {
            continue
        }

        // get index
        let t_idx = state.sorted_floors[from].iter().position(|x| *x == tech).unwrap();
        state.sorted_floors[from].remove(t_idx);
        state.sorted_floors[to].push(tech);

    }

    state.elevator = to;
    state.cost += 1;

    return state;
}

fn get_possible_states(state: &State) -> Vec<State> {

    let mut possible_new_elevator = vec![];
    if state.elevator != 0 {
        possible_new_elevator.push(state.elevator - 1);
    }
    if state.elevator != 3 {
        possible_new_elevator.push(state.elevator + 1);
    }

    // get all possible content
    let mut content = state.sorted_floors[state.elevator].clone();
    content.push(Tech::NONE);

    let combinations = content.into_iter().combinations(2).collect::<Vec<Vec<Tech>>>();

    // println!("{:?}", possible_new_elevator);
    // println!("{:?}", content);

    let mut temp_states = Vec::new();
    for e in possible_new_elevator.iter() {
        for c in combinations.iter() {
            let new_state = elevate_tech(state, c.clone(), state.elevator, *e);

            // push only if possible 
            if is_state_valid(&new_state) {
                // println!("{:?}", new_state);
                temp_states.push(new_state);
            }
        }
    }

    // check if some are going to break

    // println!("{:?}", temp_states);
    temp_states
}

fn is_end_state(state: &State) -> bool {

    if state.sorted_floors[0..3].iter().all(|x| x.is_empty()) {
        return true
    }
    return false
}

// idea : do like dijkstra/bfs (use a heap with min cost) each have a certain state.
// get a visited map that stores the cost of each state
// a state is the combination of floors (each floor sorted) and the elevator level
// do the function to sort each floor / cmp them
fn bfs(state: &mut State) -> i64 {

    let mut last_cost = 0;
    
    state.sort_floors();

    let mut heap = BinaryHeap::new();
    heap.push(state.clone());

    let mut visited = HashSet::new();
    visited.insert((state.sorted_floors.clone(), state.elevator));

    while heap.len() > 0 {
        // println!("{}", heap.len());

        let mut current_state = heap.pop().unwrap();

        // println!("{:?}", current_state);
        if current_state.cost != last_cost {
            last_cost = current_state.cost;
            println!("{}", heap.len());
            println!("{}", visited.len());
            println!("new cost : {}", last_cost);
            println!("{:?}", current_state);
        }

        if is_end_state(&current_state) {
            return current_state.cost;
        }

        let mut possible_states = get_possible_states(&current_state);

        for state in possible_states.iter_mut() {

            state.sort_floors();

            if visited.contains(&(state.sorted_floors.clone(), state.elevator)) {
                continue;
            }
            visited.insert((state.sorted_floors.clone(), state.elevator));

            heap.push(state.clone());
        }


    }

    return -1;

}

fn parse_input(input: &Vec<String>) -> Vec<Vec<Tech>> {

    let mut floors = vec![vec![]; 4];
    let r = Regex::new(r"a ([\w-]+) (microchip|generator)").unwrap();

    for i in 0..input.len() - 1 {

        let (_, tech_part) = input[i].split_once("contains").unwrap();
        let techs: Vec<&str> = tech_part.split(",").collect();
        for tech in techs {
            let caps = r.captures(tech).unwrap();
            let name = caps.get(1).unwrap().as_str().split("-").collect::<Vec<&str>>()[0].to_string();
            let t = caps.get(2).unwrap().as_str();

            let element = match t {
                "microchip" => Tech::MICROCHIP(name),
                "generator" => Tech::RTG(name),
                _ => panic!("Unknown")
            };

            floors[i].push(element);
        }

    }

    floors
}

pub fn part2(input: &Vec<String>) -> i64 {

    let mut floors = parse_input(input);

    let elevator: usize = 0; // 0 -> 3

    let mut init_state = State {
        sorted_floors: floors,
        elevator,
        cost: 0,
    };

    init_state.sorted_floors[0].push(Tech::RTG(String::from("elerium")));
    init_state.sorted_floors[0].push(Tech::MICROCHIP(String::from("elerium")));
    init_state.sorted_floors[0].push(Tech::RTG(String::from("dilithium")));
    init_state.sorted_floors[0].push(Tech::MICROCHIP(String::from("dilithium")));


    let res = bfs(&mut init_state);

    return res;
}