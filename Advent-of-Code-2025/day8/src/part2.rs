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

    fn get_largest_circuit_size(&mut self) -> usize {
        let circuit_sizes = self.get_circuit_sizes();
        *circuit_sizes.iter().max().unwrap()
    }
}

pub fn part2(input: &Vec<String>) -> i64 {
    let positions = parse_input(input);
    let num_positions = positions.len();
    let n = positions.len();
    let mut result = 0;
    
    let mut uf = UnionFind::new(n);
    
    let mut pairs: Vec<(usize, usize, f64)> = Vec::new();
    for i in 0..n {
        for j in (i + 1)..n {
            let dist = distance(&positions[i], &positions[j]);
            pairs.push((i, j, dist));
        }
    }
    
    pairs.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());
    
    for (i, j, _dist) in pairs.iter() {
        uf.union(*i, *j);

        if uf.get_largest_circuit_size() == num_positions {
            let (p1, p2) = (positions[*i].clone(), positions[*j].clone());
            result = p1.x * p2.x;
            break;
        }
    }

    return result;
}