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

pub fn part1(input: &Vec<String>) -> i64 {
    let (shapes, regions) = parse_input(input);
    let mut count = 0;
    
    for region in &regions {
        let region_area = region.width * region.height;
        
        let mut required_area = 0;
        for (shape_idx, &count) in region.present_counts.iter().enumerate() {
            let shape = &shapes[shape_idx];
            // Count '#' cells in the shape
            let shape_area: usize = shape.grid.iter()
                .flat_map(|row| row.iter())
                .filter(|&&cell| cell)
                .count();
            required_area += shape_area * count;
        }
        
        if required_area <= region_area {
            count += 1;
        }
    }
    
    return count;
}