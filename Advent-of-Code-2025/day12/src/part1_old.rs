#[derive(Debug, Clone)]
pub struct Shape {
    pub index: usize,
    pub grid: Vec<Vec<bool>>, // true = '#', false = '.'
    pub width: usize,
    pub height: usize,
}

#[derive(Debug)]
pub struct Region {
    pub width: usize,
    pub height: usize,
    pub present_counts: Vec<usize>, // Quantity of each shape index
}

pub fn parse_input(input: &Vec<String>) -> (Vec<Shape>, Vec<Region>) {
    let mut shapes: Vec<Shape> = Vec::new();
    let mut regions: Vec<Region> = Vec::new();
    
    let mut i = 0;
    
    while i < input.len() {
        let line = &input[i];
        
        if line.contains(':') && !line.contains('x') {
            let parts: Vec<&str> = line.split(':').collect();
            let index = parts[0].trim().parse::<usize>().unwrap();
            
            i += 1;
            let mut grid: Vec<Vec<bool>> = Vec::new();
            
            while i < input.len() && !input[i].is_empty() && !input[i].contains(':') {
                let row: Vec<bool> = input[i].chars().map(|c| c == '#').collect();
                grid.push(row);
                i += 1;
            }
            
            let height = grid.len();
            let width = if height > 0 { grid[0].len() } else { 0 };
            
            shapes.push(Shape {
                index,
                grid,
                width,
                height,
            });
        }

        else if line.contains('x') && line.contains(':') {
            let parts: Vec<&str> = line.split(':').collect();
            let dimensions: Vec<&str> = parts[0].trim().split('x').collect();
            let width = dimensions[0].parse::<usize>().unwrap();
            let height = dimensions[1].parse::<usize>().unwrap();
            
            let counts_str = parts[1].trim();
            let present_counts: Vec<usize> = counts_str
                .split_whitespace()
                .map(|s| s.parse::<usize>().unwrap())
                .collect();
            
            regions.push(Region {
                width,
                height,
                present_counts,
            });
            
            i += 1;
        }
        else {
            i += 1;
        }
    }
    
    return (shapes, regions);
}

fn get_all_transformations(shape: &Shape) -> Vec<Shape> {
    let mut transformations = Vec::new();
    let mut seen = std::collections::HashSet::new();
    
    // Original
    let original = shape.clone();
    
    // Generate all 8 possible transformations
    for flip in [false, true] {
        let mut current = if flip { flip_horizontal(&original) } else { original.clone() };
        
        for _ in 0..4 {
            // Convert grid to string for deduplication
            let grid_str = grid_to_string(&current);
            if !seen.contains(&grid_str) {
                seen.insert(grid_str);
                transformations.push(current.clone());
            }
            current = rotate_90(&current);
        }
    }
    
    transformations
}

fn rotate_90(shape: &Shape) -> Shape {
    let old_height = shape.grid.len();
    let old_width = if old_height > 0 { shape.grid[0].len() } else { 0 };
    
    let new_width = old_height;
    let new_height = old_width;
    
    let mut new_grid = vec![vec![false; new_width]; new_height];
    
    for row in 0..old_height {
        for col in 0..old_width {
            new_grid[col][old_height - 1 - row] = shape.grid[row][col];
        }
    }
    
    Shape {
        index: shape.index,
        grid: new_grid,
        width: new_width,
        height: new_height,
    }
}

fn flip_horizontal(shape: &Shape) -> Shape {
    let mut new_grid = shape.grid.clone();
    for row in new_grid.iter_mut() {
        row.reverse();
    }
    
    Shape {
        index: shape.index,
        grid: new_grid,
        width: shape.width,
        height: shape.height,
    }
}

fn grid_to_string(shape: &Shape) -> String {
    shape.grid.iter()
        .map(|row| row.iter().map(|&b| if b { '#' } else { '.' }).collect::<String>())
        .collect::<Vec<_>>()
        .join("\n")
}

fn can_place(grid: &Vec<Vec<bool>>, shape: &Shape, start_row: usize, start_col: usize) -> bool {
    let grid_height = grid.len();
    let grid_width = if grid_height > 0 { grid[0].len() } else { 0 };
    
    // Check if shape fits within grid bounds
    if start_row + shape.height > grid_height || start_col + shape.width > grid_width {
        return false;
    }
    
    // Check if any '#' cells overlap with already occupied cells
    for row in 0..shape.height {
        for col in 0..shape.width {
            if shape.grid[row][col] {  // If this is a '#' cell in the shape
                if grid[start_row + row][start_col + col] {  // And the grid cell is occupied
                    return false;
                }
            }
        }
    }
    
    true
}

fn place(grid: &mut Vec<Vec<bool>>, shape: &Shape, start_row: usize, start_col: usize, occupy: bool) {
    for row in 0..shape.height {
        for col in 0..shape.width {
            if shape.grid[row][col] {  // Only place '#' cells
                grid[start_row + row][start_col + col] = occupy;
            }
        }
    }
}

fn can_fit_all_presents(region: &Region, shapes: &Vec<Shape>) -> bool {
    let mut grid = vec![vec![false; region.width]; region.height];
    let mut presents_to_place = Vec::new();
    
    // Build list of all presents to place
    for (shape_idx, &count) in region.present_counts.iter().enumerate() {
        for _ in 0..count {
            presents_to_place.push(shape_idx);
        }
    }
    
    backtrack(&mut grid, &presents_to_place, 0, shapes)
}

fn backtrack(
    grid: &mut Vec<Vec<bool>>,
    presents: &Vec<usize>,
    present_idx: usize,
    shapes: &Vec<Shape>
) -> bool {
    // Base case: all presents placed
    if present_idx == presents.len() {
        return true;
    }
    
    let shape_idx = presents[present_idx];
    let shape = &shapes[shape_idx];
    
    // Try all rotations/flips
    for transformed_shape in get_all_transformations(shape) {
        // Try all positions in grid
        for row in 0..grid.len() {
            for col in 0..grid[0].len() {
                if can_place(&grid, &transformed_shape, row, col) {
                    place(grid, &transformed_shape, row, col, true);
                    
                    if backtrack(grid, presents, present_idx + 1, shapes) {
                        return true;
                    }
                    
                    // Backtrack
                    place(grid, &transformed_shape, row, col, false);
                }
            }
        }
    }
    
    false
}

pub fn part1(input: &Vec<String>) -> i64 {
    let (shapes, regions) = parse_input(input);
    let mut count = 0;
    for region in &regions {
        println!("Checking region: {}x{}", region.width, region.height);
        if can_fit_all_presents(region, &shapes) {
            count += 1;
        }
    }
    
    return count;
}