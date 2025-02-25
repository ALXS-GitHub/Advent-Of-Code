use std::collections::HashMap;

struct Layer {
    range: i64,
    scanner_pos: i64,
    inc: bool,
}

impl Layer {
    fn step(&mut self) {
        self.scanner_pos += if self.inc { 1 } else { -1 };
        if self.scanner_pos == 0 {
            self.inc = true;
        } else if self.scanner_pos == self.range - 1 {
            self.inc = false;
        }
    }
}

fn parse_input(input: &Vec<String>) -> HashMap<i64, Layer> {

    let mut layers = HashMap::new();

    for line in input {
        let (id, range) = line.split_once(": ").unwrap();
        let (id, range) = (
            id.parse::<i64>().unwrap(),
            range.parse::<i64>().unwrap()
        );

        layers.insert(id, Layer {
            range,
            scanner_pos: 0, 
            inc: true
        });
    }

    layers
}

fn get_severity(layers: &mut HashMap<i64, Layer>, max_depth: i64) -> i64 {
    let mut severity = 0;
    
    for packet in 0..=max_depth {
        if let Some((depth, layer)) = layers.get_key_value(&packet) {
            if layer.scanner_pos == 0 {
                severity += depth * layer.range;
            }
        }

        for (_, l) in layers.iter_mut() {
            l.step();
        }

    }

    severity
}

pub fn part1(input: &Vec<String>) -> i64 {

    let mut layers = parse_input(input);
    let max_depth = input[input.len() - 1].split(": ").collect::<Vec<_>>()[0].parse::<i64>().unwrap();

    let severity = get_severity(&mut layers, max_depth);

    return severity;
}