use std::collections::HashMap;

#[derive(Debug, Clone)]
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

fn get_severity(layers: &mut HashMap<i64, Layer>, delay: i64) -> i64 {

    for (depth, layer) in layers.iter() {
        if (depth + delay) % (2 * (layer.range - 1)) == 0 {
            return 1
        }
    }
    0
}

pub fn part2(input: &Vec<String>) -> i64 {

    let mut layers = parse_input(input);

    let mut delay = 0;

    loop {
        // let mut layers_clone = layers.clone();
        let severity = get_severity(&mut layers, delay);

        if severity == 0 {
            break
        }

        // for (_, layer) in layers.iter_mut() {
        //     layer.step();
        // }
        delay += 1;

    }
    return delay;
}