pub struct Pos {
    pub x: i64,
    pub y: i64,
}

pub fn parse_input(input: &Vec<String>) -> Vec<Pos> {
    let mut positions: Vec<Pos> = Vec::new();

    for line in input {
        let coords: Vec<&str> = line.split(',').collect();
        let x: i64 = coords[0].trim().parse().unwrap();
        let y: i64 = coords[1].trim().parse().unwrap();
        positions.push(Pos { x, y });
    }

    return positions;
}

pub fn area(p1: &Pos, p2: &Pos) -> i64 {
    let width = (p1.x - p2.x).abs() + 1;
    let height = (p1.y - p2.y).abs() + 1;
    return width * height;
}

fn is_on_segment(px: i64, py: i64, x1: i64, y1: i64, x2: i64, y2: i64) -> bool {
    if x1 == x2 {
        return px == x1 && py >= y1.min(y2) && py <= y1.max(y2);
    } else {
        return py == y1 && px >= x1.min(x2) && px <= x1.max(x2);
    }
}

// Check if point is on the polygon boundary (red or green tiles)
fn is_on_boundary(px: i64, py: i64, positions: &Vec<Pos>) -> bool {
    for i in 0..positions.len() {
        let p1 = &positions[i];
        let p2 = &positions[(i + 1) % positions.len()];
        if is_on_segment(px, py, p1.x, p1.y, p2.x, p2.y) {
            return true;
        }
    }
    false
}

// Ray casting algorithm to check if a point is inside a polygon
fn is_inside_polygon(px: i64, py: i64, positions: &Vec<Pos>) -> bool {
    let mut inside = false;
    let n = positions.len();
    
    for i in 0..n {
        let j = (i + 1) % n;
        let xi = positions[i].x;
        let yi = positions[i].y;
        let xj = positions[j].x;
        let yj = positions[j].y;
        
        // Check if horizontal ray from point intersects this edge
        // source : https://stackoverflow.com/questions/22521982/check-if-point-is-inside-a-polygon
        if ((yi > py) != (yj > py)) && (px < (xj - xi) * (py - yi) / (yj - yi) + xi) {
            inside = !inside;
        }
    }
    
    inside
}

fn is_valid_tile(px: i64, py: i64, positions: &Vec<Pos>) -> bool {
    is_on_boundary(px, py, positions) || is_inside_polygon(px, py, positions)
}

fn check_rectangle_corners(x1: i64, y1: i64, x2: i64, y2: i64, positions: &Vec<Pos>) -> bool {
    let min_x = x1.min(x2);
    let max_x = x1.max(x2);
    let min_y = y1.min(y2);
    let max_y = y1.max(y2);
    
    is_valid_tile(min_x, min_y, positions)
        && is_valid_tile(min_x, max_y, positions)
        && is_valid_tile(max_x, min_y, positions)
        && is_valid_tile(max_x, max_y, positions)
}

// Check if rectangle boundary edges intersect with or are contained by the polygon
fn is_rectangle_valid(x1: i64, y1: i64, x2: i64, y2: i64, positions: &Vec<Pos>) -> bool {
    let min_x = x1.min(x2);
    let max_x = x1.max(x2);
    let min_y = y1.min(y2);
    let max_y = y1.max(y2);
    
    if !check_rectangle_corners(x1, y1, x2, y2, positions) {
        return false;
    }
    
    // Sample points along the edges to verify they're all valid
    // Sample at most 100 points per edge
    let step = ((max_x - min_x) / 100).max(1); 
    for x in (min_x..=max_x).step_by(step as usize) {
        if !is_valid_tile(x, min_y, positions) || !is_valid_tile(x, max_y, positions) {
            return false;
        }
    }
    
    let step = ((max_y - min_y) / 100).max(1);
    for y in (min_y..=max_y).step_by(step as usize) {
        if !is_valid_tile(min_x, y, positions) || !is_valid_tile(max_x, y, positions) {
            return false;
        }
    }
    
    // Sample some interior points (seems to be unnecessary)
    // let x_step = ((max_x - min_x) / 100).max(1);
    // let y_step = ((max_y - min_y) / 100).max(1);
    // for x in (min_x..=max_x).step_by(x_step as usize) {
    //     for y in (min_y..=max_y).step_by(y_step as usize) {
    //         if !is_valid_tile(x, y, positions) {
    //             return false;
    //         }
    //     }
    // }
    
    true
}

pub fn part2(input: &Vec<String>) -> i64 {

    let positions = parse_input(input);

    let mut max_area = 0;
    
    for i in 0..positions.len() {
        for j in (i + 1)..positions.len() {
            let p1 = &positions[i];
            let p2 = &positions[j];

            let current_area = area(p1, p2);
            if current_area <= max_area {
                continue;
            }
            
            if is_rectangle_valid(p1.x, p1.y, p2.x, p2.y, &positions) {
                max_area = max_area.max(current_area);
            }
            
        }
    }

    return max_area;
}