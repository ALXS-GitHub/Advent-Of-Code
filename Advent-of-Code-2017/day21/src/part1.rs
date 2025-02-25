use std::fmt::{Display, Formatter};
use std::collections::HashMap;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Pattern {
    grid: Vec<Vec<char>>
}

impl Pattern {

    fn new(grid: Vec<Vec<char>>) -> Self {
        Pattern {
            grid
        }
    }

    fn size(&self) -> usize {
        return self.grid.len();
    }

    fn from_grid(grid: &Vec<Vec<char>>) -> Vec<Vec<Pattern>> {
        let size;
        if grid.len() % 2 == 0 {
            size = 2;
        } else if grid.len() % 3 == 0 {
            size = 3;
        } else {
            panic!("Grid impossible to split")
        }
        let mut patterns = vec![];
        let n = grid.len();

        for row in (0..n).step_by(size) {
            let mut pattern_row = vec![];
            for col in (0..n).step_by(size) {
                let mut sub_pattern = vec![vec!['.'; size]; size];
                
                for i in 0..size {
                    for j in 0..size {
                        sub_pattern[i][j] = grid[row + i][col + j];
                    }
                }
                
                pattern_row.push(Pattern::new(sub_pattern));
            }
            patterns.push(pattern_row);
        }


        patterns
    }

    fn to_grid(patterns: &Vec<Vec<Pattern>>) -> Vec<Vec<char>> {
        let size = patterns[0][0].size();
        let mut grid = vec![vec!['.'; patterns.len() * size]; patterns.len() * size];
        for i in 0..patterns.len() {
            for j in 0..patterns.len() {

                for row in 0..size {
                    for col in 0..size {
                        grid[i*size + row][j*size + col] = patterns[i][j].grid[row][col];
                    }
                }

            }
        }
        grid

    }

    fn next(&self, instructions: &HashMap<Pattern, Pattern>) -> Pattern {
        let resulting_pattern = instructions.get(self);
        match resulting_pattern {
            Some(p) => {
                return p.clone()
            },
            None => {
                panic!("Unexpected")
            }
        };
    }

    fn from_string(string: &str) -> Self {
        let grid = string.split("/").collect::<Vec<_>>().iter().map(|x| x.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
        Pattern::new(grid)
    }

    fn flip(&self) -> Pattern {
        let mut grid = self.grid.clone();
        grid.reverse();
        Pattern::new(grid)
    }

    fn rotate(&self) -> Pattern {
        let grid = self.grid.clone();
        let n = grid.len();
        let mut new_grid = vec![vec!['.'; n]; n];
        for i in 0..n {
            for j in 0..n {
                new_grid[i][j] = grid[n - j - 1][i];
            }
        }
        Pattern::new(new_grid)
    }

    fn get_all_variants(&self) -> Vec<Pattern> {

        let mut variants = vec![];

        let mut pattern = self.clone();
        let mut flipped = pattern.flip();

        variants.push(pattern.clone());
        variants.push(flipped.clone());

        for _ in 0..3 {
            pattern = pattern.rotate();
            flipped = flipped.rotate();

            variants.push(pattern.clone());
            variants.push(flipped.clone());
        }

        variants
    }
}

fn count_lights(grid: &Vec<Vec<char>>) -> i64 {
    let mut count = 0;
        for l in grid {
            for c in l {
                if *c == '#' {
                    count += 1;
                }
            }
        }
        count
}

impl Display for Pattern {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        for row in &self.grid {
            for cell in row {
                write!(f, "{}", cell)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

fn parse_input(input: &Vec<String>) -> HashMap<Pattern, Pattern> {
    let mut instructions = HashMap::new();

    for line in input {
        let (from, to) = line.split_once(" => ").unwrap();
        let from = Pattern::from_string(from);
        let to = Pattern::from_string(to);
        let froms = from.get_all_variants();
        for f in froms {
            let v = instructions.get_mut(&f);
            match v {
                Some(v) => {
                    if *v != to {
                        panic!("Should not have duplicates with different outputs")
                    }
                }, 
                None => {instructions.insert(f, to.clone());}
            }
        }
    }

    instructions
}

fn get_lights_on(grid: &mut Vec<Vec<char>>, iter: usize, instructions: &HashMap<Pattern, Pattern>) -> i64 {

    if iter == 0 {
        return count_lights(grid)
    }
    
    let mut patterns = Pattern::from_grid(&grid);
    for i in 0..patterns.len() {
        for j in 0..patterns.len() {
            patterns[i][j] = patterns[i][j].next(instructions);
        }
    }
    *grid = Pattern::to_grid(&patterns);

    return get_lights_on(grid, iter - 1, instructions);

}


pub fn part1(input: &Vec<String>) -> i64 {

    let instructions = parse_input(input);
    let pattern = Pattern::new(vec![
        vec!['.', '#', '.'],
        vec!['.', '.', '#'],
        vec!['#', '#', '#']
    ]);
    let mut grid = Pattern::to_grid(&vec![vec![pattern]]);
    
    
    let iter = 5;
    let res = get_lights_on(&mut grid, iter, &instructions);

    return res;
}