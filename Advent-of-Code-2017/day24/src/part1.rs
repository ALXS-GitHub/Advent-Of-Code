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

fn get_strongest(bridge: &mut Bridge, remaining_components: &mut Vec<Component>) -> i32 {

    let mut max = bridge.strength();

    for i in 0..remaining_components.len() {
        let r = remaining_components[i];
        match bridge.insert(r) {
            true => {
                remaining_components.remove(i);
                max = get_strongest(bridge, remaining_components).max(max);
                remaining_components.insert(i, r);
                bridge.pop();
            }
            _ => {}
        }
    }

    max

}

pub fn part1(input: &Vec<String>) -> i64 {

    let mut bridge = Bridge::new();
    let mut remaining_components = parse_input(input);

    let res = get_strongest(&mut bridge, &mut remaining_components);

    return res as i64;
}