pub fn part2(input: &Vec<String>) -> i64 {
    let mut total = 0;

    for line in input.iter() {
        let parts: Vec<i64> = line.split("x").map(|x| x.parse::<i64>().unwrap()).collect();
        let (l, w, h) = (parts[0], parts[1], parts[2]);

        let mut dims = vec![l, w, h];
        let max_index = dims.iter().position(|&x| x == *dims.iter().max().unwrap()).unwrap();
        dims.remove(max_index);

        total += 2*dims[0] + 2*dims[1];
        total += l*w*h;

    }

    return total;
}