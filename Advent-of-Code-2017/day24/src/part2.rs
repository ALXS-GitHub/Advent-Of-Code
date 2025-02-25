#[derive(Debug, Clone, Copy)]
struct Component {
    a: i32,
    b: i32,
}

impl Component {
    fn has(&self, n: i32) -> bool {
        return self.a == n || self.b == n;
    }
}

#[derive(Debug, Clone)]
struct Bridge {
    end: i32, 
    components: Vec<Component>,
}

impl Bridge {
    fn new() -> Self {
        Self {
            end: 0,
            components: Vec::new()
        }
    }

    fn strength(&self) -> i32 {
        let mut sum = 0;

        for c in self.components.iter() {
            sum += c.a + c.b
        }

        sum
    }

    fn len(&self) -> usize {
        return self.components.len();
    }

    fn insert(&mut self, component: Component) -> bool {
        if !component.has(self.end) {
            return false
        }

        self.components.push(component);
        self.end = if component.a == self.end { component.b } else { component.a };

        true
    }

    fn pop(&mut self) -> Option<Component> {
        let last = self.components.pop();
        if let Some(c) = last {
            self.end = if c.a == self.end { c.b } else { c.a };
        }

        last
    }
}

fn parse_input(input: &Vec<String>) -> Vec<Component> {
    let mut components = vec![];

    for line in input {
        let parts = line.split_once("/").unwrap();
        components.push(Component {
            a: parts.0.parse::<i32>().unwrap(),
            b: parts.1.parse::<i32>().unwrap()
        })
    }

    components
}

fn get_strongest_longest(bridge: &mut Bridge, remaining_components: &mut Vec<Component>) -> (usize, i32) {

    let mut max = bridge.strength();
    let mut max_len = bridge.len();

    for i in 0..remaining_components.len() {
        let r = remaining_components[i];
        match bridge.insert(r) {
            true => {
                remaining_components.remove(i);
                let sub = get_strongest_longest(bridge, remaining_components);
                if sub.0 > max_len {
                    max_len = sub.0;
                    max = sub.1;
                } else if sub.0 == max_len {
                    if sub.1 > max {
                        max = sub.1;
                    }
                }
                remaining_components.insert(i, r);
                bridge.pop();
            }
            _ => {}
        }
    }

    (max_len, max)

}

pub fn part2(input: &Vec<String>) -> i64 {

    let mut bridge = Bridge::new();
    let mut remaining_components = parse_input(input);

    let res = get_strongest_longest(&mut bridge, &mut remaining_components);

    return res.1 as i64;
}