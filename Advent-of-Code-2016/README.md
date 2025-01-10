# [⛄ Advent Of Code 2016 ⛄](https://adventofcode.com/2016)

I am doing the 2016 edition of the Advent of Code years later, just for fun and to improve my problem solving skills. 
I am using Rust to solve most of the challenges, and I will try to optimize the code as much as possible.

## Usage

You can run my solutions by running the following commands:

```sh
cd day<day_number>
cargo run
```

If needed you can use the `--release` flag to run the optimized version of the code.

To run a specific part, use the following commands:

```sh
cargo run -- --part1  # Run part 1 only
cargo run -- --part2  # Run part 2 only
```

If no arguments are provided, both parts will be run by default.

In the output, you will see the answers and the time it took for each part to run.

Make sure that the test input provided in the problem is set in the `inputtest.txt` file in the `day<day_number>` folder, and the real input is set in the `input.txt` file.

## Benchmarks

By default, when running the code, a benchmark for the time will be run as well. However this approach is very simple and not very accurate. If you want to run a more accurate benchmark, you can use the following command:

```sh
cargo bench
```

This will run benchmarks using the `criterion` crate and will give you a more accurate time for each part.

## Tests

Unit tests are provided in the `lib.rs` file for each day. Make sure you set the correct result for each test before running them.
There are both test for the test input provided in the problem and for the real input.

To run the tests, use the following command:

```sh
cargo test --lib
```

## Fetching the input

If there is no `input.txt` file for the current day, the code will automatically fetch the input from the Advent of Code website. You will need to provide your session cookie in the `.env` file in the root of the project as `SESSION_TOKEN`. The cookie can be found in the browser after logging in to the website.

In the `main.rs` file you should also set the `year` variable to the correct year of the challenge. If needed you can also do this manually for the day, but in the current template it will get the day automatically according to the folder name (with the following format : `day<day_number>`).

If the `input.txt` file is already present, the code will not fetch the input again, so you can run the code offline. However, if you want to fetch the input again, you can use the following command:

```sh
cargo run -- --fetch
```

## Using the test input file

If you first want to test your solution with a given test input file, you can create manually a `inputtest.txt` file in the `day<day_number>` folder and set the test input there. Then you can run the code with the following command:

```sh
cargo run -- --test
```

This command will run the code with the test input file instead of the real input file.

## Solutions

The solutions for each day can be found in the following table. The time are being calculated using the `criterion` crate for accurate benchmarks. Note that most of the challenges are done without any parallelism, except for some days where I judged it was necessary to get faster results.

| 🗓️Day🗓️ | ⛄Part 1 Solution⛄ | 🎁Part 2 Solution🎁 | ❄️Part 1 Time❄️ | 🎄Part 2 Time🎄 |
|:-------:|:------------------:|:------------------:|:--------------:|:--------------:|
| 🗓️1🗓️ | ⛄[/day1/src/part1.rs](/day1/src/part1.rs)⛄ | 🎁[/day1/src/part2.rs](/day1/src/part2.rs)🎁 | ❄️16.009 µs❄️ | 🎄36.355 µs🎄 |
| 🗓️2🗓️ | ⛄[/day2/src/part1.rs](/day2/src/part1.rs)⛄ | 🎁[/day2/src/part2.rs](/day2/src/part2.rs)🎁 | ❄️19.598 µs❄️ | 🎄21.644 µs🎄 |
| 🗓️3🗓️ | ⛄[/day3/src/part1.rs](/day3/src/part1.rs)⛄ | 🎁[/day3/src/part2.rs](/day3/src/part2.rs)🎁 | ❄️978.26 µs❄️ | 🎄978.19 µs🎄 |
| 🗓️4🗓️ | ⛄[/day4/src/part1.rs](/day4/src/part1.rs)⛄ | 🎁[/day4/src/part2.rs](/day4/src/part2.rs)🎁 | ❄️1.3500 ms❄️ | 🎄722.10 µs🎄 |
| 🗓️5🗓️ | ⛄[/day5/src/part1.rs](/day5/src/part1.rs)⛄ | 🎁[/day5/src/part2.rs](/day5/src/part2.rs)🎁 | ❄️2.2291 s❄️ | 🎄6.5611 s🎄 |
| 🗓️6🗓️ | ⛄[/day6/src/part1.rs](/day6/src/part1.rs)⛄ | 🎁[/day6/src/part2.rs](/day6/src/part2.rs)🎁 | ❄️47.028 µs❄️ | 🎄44.357 µs🎄 |
| 🗓️7🗓️ | ⛄[/day7/src/part1.rs](/day7/src/part1.rs)⛄ | 🎁[/day7/src/part2.rs](/day7/src/part2.rs)🎁 | ❄️702.48 µs❄️ | 🎄1.0949 ms🎄 |
| 🗓️8🗓️ | ⛄[/day8/src/part1.rs](/day8/src/part1.rs)⛄ | 🎁[/day8/src/part2.rs](/day8/src/part2.rs)🎁 | ❄️467.08 µs❄️ | 🎄422.02 µs🎄 |
| 🗓️9🗓️ | ⛄[/day9/src/part1.rs](/day9/src/part1.rs)⛄ | 🎁[/day9/src/part2.rs](/day9/src/part2.rs)🎁 | ❄️42.222 µs❄️ | 🎄383.99 µs🎄 |
| 🗓️10🗓️ | ⛄[/day10/src/part1.rs](/day10/src/part1.rs)⛄ | 🎁[/day10/src/part2.rs](/day10/src/part2.rs)🎁 | ❄️3.1812 ms❄️ | 🎄3.2382 ms🎄 |
| 🗓️11🗓️ | ⛄[/day11/src/part1.rs](/day11/src/part1.rs)⛄ | 🎁[/day11/src/part2.rs](/day11/src/part2.rs)🎁 | ❄️3.5 s❄️ | 🎄⚠️+120 s🎄 |

Notes : Day 11 was a mess. I used a brute force approach for part 2 (as for part1) and it took a long time to run. I still looking for an optimized solution for this problem.



## Template

Do you want to use my rust template for your own solutions? 

Then just get the `template` folder from this repository and use the following command to create a new day:

```sh
cargo generate --path ./template
```

Then it will ask you for the project name, just enter the name you want to give for the day and it will create a new folder with the template code for you.

## Author

⛄ [ALXS](https://github.com/ALXS-GitHub)
