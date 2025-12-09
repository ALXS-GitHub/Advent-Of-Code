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

// Check if a rectangle intersects with a segment
fn rectangle_intersects_segment(rect_x1: i64, rect_y1: i64, rect_x2: i64, rect_y2: i64,
                                 seg_x1: i64, seg_y1: i64, seg_x2: i64, seg_y2: i64) -> bool {
    let rect_min_x = rect_x1.min(rect_x2);
    let rect_max_x = rect_x1.max(rect_x2);
    let rect_min_y = rect_y1.min(rect_y2);
    let rect_max_y = rect_y1.max(rect_y2);
    
    if seg_x1 == seg_x2 {
        
        let seg_min_y = seg_y1.min(seg_y2);
        let seg_max_y = seg_y1.max(seg_y2);
        
        return seg_x1 > rect_min_x && seg_x1 < rect_max_x &&
               seg_max_y > rect_min_y && rect_max_y > seg_min_y;
    } else {
        let seg_min_x = seg_x1.min(seg_x2);
        let seg_max_x = seg_x1.max(seg_x2);
        
        return seg_y1 > rect_min_y && seg_y1 < rect_max_y &&
               seg_max_x > rect_min_x && rect_max_x > seg_min_x;
    }
}

fn is_rectangle_valid(x1: i64, y1: i64, x2: i64, y2: i64, positions: &Vec<Pos>) -> bool {
    for i in 0..positions.len() {
        let p1 = &positions[i];
        let p2 = &positions[(i + 1) % positions.len()];
        
        if rectangle_intersects_segment(x1, y1, x2, y2, p1.x, p1.y, p2.x, p2.y) {
            return false;
        }
    }
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
                max_area = current_area;
            }
        }
    }

    return max_area;
}