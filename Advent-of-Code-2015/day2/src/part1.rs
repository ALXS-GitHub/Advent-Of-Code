pub fn part1(input: &Vec<String>) -> i64 {

    let mut total = 0;

    for line in input.iter() {
        let parts: Vec<i64> = line.split("x").map(|x| x.parse::<i64>().unwrap()).collect();
        let (l, w, h) = (parts[0], parts[1], parts[2]);

        let sides = vec![l*w, w*h, h*l];

        let min = *sides.iter().min().unwrap();

        total += 2*sides[0] + 2*sides[1] + 2*sides[2] + min;

    }

    return total;
}