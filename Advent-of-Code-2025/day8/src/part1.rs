use std::collections::HashMap;

#[derive(Clone, Debug, Hash)]
pub struct Pos {
    x: i64,
    y: i64,
    z: i64,
}

impl PartialEq for Pos {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}
impl Eq for Pos {}

pub fn parse_input(input: &Vec<String>) -> Vec<Pos> {
    let mut positions: Vec<Pos> = Vec::new();

    for line in input {
        let coords: Vec<i64> = line
            .split(',')
            .map(|s| s.parse::<i64>().unwrap())
            .collect();
        positions.push(Pos {
            x: coords[0],
            y: coords[1],
            z: coords[2],
        });
    }

    return positions;
}

pub fn distance(a: &Pos, b: &Pos) -> f64 {
    let dx = (a.x - b.x) as f64;
    let dy = (a.y - b.y) as f64;
    let dz = (a.z - b.z) as f64;

    return (dx * dx + dy * dy + dz * dz).sqrt();
}

// Union-Find structure
struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x != root_y {
            // Union by size
            if self.size[root_x] < self.size[root_y] {
                self.parent[root_x] = root_y;
                self.size[root_y] += self.size[root_x];
            } else {
                self.parent[root_y] = root_x;
                self.size[root_x] += self.size[root_y];
            }
        }
    }

    fn get_circuit_sizes(&mut self) -> Vec<usize> {
        let mut circuit_sizes: HashMap<usize, usize> = HashMap::new();
        for i in 0..self.parent.len() {
            let root = self.find(i);
            *circuit_sizes.entry(root).or_insert(0) += 1;
        }
        circuit_sizes.values().copied().collect()
    }
}

pub fn part1(input: &Vec<String>) -> i64 {
    let positions = parse_input(input);
    let n = positions.len();
    let connections = 1000;
    let num_largest_to_consider = 3;
    
    let mut uf = UnionFind::new(n);
    
    let mut pairs: Vec<(usize, usize, f64)> = Vec::new();
    for i in 0..n {
        for j in (i + 1)..n {
            let dist = distance(&positions[i], &positions[j]);
            pairs.push((i, j, dist));
        }
    }
    
    pairs.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());
    
    let mut connections_made = 0;
    for (i, j, _dist) in pairs.iter() {
        if connections_made >= connections {
            break;
        }
        uf.union(*i, *j);
        connections_made += 1;
    }
    
    let mut circuit_sizes = uf.get_circuit_sizes();
    circuit_sizes.sort_by(|a, b| b.cmp(a));
    
    let mut total_size = 1;
    for i in 0..num_largest_to_consider.min(circuit_sizes.len()) {
        total_size *= circuit_sizes[i] as i64;
    }

    return total_size;
}