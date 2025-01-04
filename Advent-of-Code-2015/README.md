# [⛄ Advent Of Code 2015 ⛄](https://adventofcode.com/2015)

I am doing the 2015 edition of the Advent of Code years later, just for fun and to improve my problem solving skills. 
I am using Rust to solve most of the challenges, and I will try to optimize the code as much as possible.

I am using the same template I used for year 2024.

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

## Solutions

The solutions for each day can be found in the following table. The time are being calculated using the `criterion` crate for accurate benchmarks. Note that most of the challenges are done without any parallelism, except for some days where I judged it was necessary to get faster results.

| 🗓️Day🗓️ | ⛄Part 1 Solution⛄ | 🎁Part 2 Solution🎁 | ❄️Part 1 Time❄️ | 🎄Part 2 Time🎄 |
|:-------:|:------------------:|:------------------:|:--------------:|:--------------:|
| 🗓️1🗓️ | ⛄[/day1/src/part1.rs](/day1/src/part1.rs)⛄ | 🎁[/day1/src/part2.rs](/day1/src/part2.rs)🎁 | ❄️3.8574 µs❄️ | 🎄1.7759 µs🎄 |
| 🗓️2🗓️ | ⛄[/day2/src/part1.rs](/day2/src/part1.rs)⛄ | 🎁[/day2/src/part2.rs](/day2/src/part2.rs)🎁 | ❄️120.51 µs❄️ | 🎄129.06 µs🎄 |
| 🗓️3🗓️ | ⛄[/day3/src/part1.rs](/day3/src/part1.rs)⛄ | 🎁[/day3/src/part2.rs](/day3/src/part2.rs)🎁 | ❄️294.24 µs❄️ | 🎄314.37 µs🎄 |
| 🗓️4🗓️ | ⛄[/day4/src/part1.rs](/day4/src/part1.rs)⛄ | 🎁[/day4/src/part2.rs](/day4/src/part2.rs)🎁 | ❄️80.596 ms❄️ | 🎄522.50 ms🎄 |
| 🗓️5🗓️ | ⛄[/day5/src/part1.rs](/day5/src/part1.rs)⛄ | 🎁[/day5/src/part2.rs](/day5/src/part2.rs)🎁 | ❄️1.2294 ms❄️ | 🎄2.4733 ms🎄 |
| 🗓️6🗓️ | ⛄[/day6/src/part1.rs](/day6/src/part1.rs)⛄ | 🎁[/day6/src/part2.rs](/day6/src/part2.rs)🎁 | ❄️228.02 ms❄️ | 🎄314.54 ms🎄 |
| 🗓️7🗓️ | ⛄[/day7/src/part1.rs](/day7/src/part1.rs)⛄ | 🎁[/day7/src/part2.rs](/day7/src/part2.rs)🎁 | ❄️169.05 µs❄️ | 🎄222.28 µs🎄 |
| 🗓️8🗓️ | ⛄[/day8/src/part1.rs](/day8/src/part1.rs)⛄ | 🎁[/day8/src/part2.rs](/day8/src/part2.rs)🎁 | ❄️237.32 µs❄️ | 🎄75.442 µs🎄 |
| 🗓️9🗓️ | ⛄[/day9/src/part1.rs](/day9/src/part1.rs)⛄ | 🎁[/day9/src/part2.rs](/day9/src/part2.rs)🎁 | ❄️52.736 ms❄️ | 🎄54.076 ms🎄 |
| 🗓️10🗓️ | ⛄[/day10/src/part1.rs](/day10/src/part1.rs)⛄ | 🎁[/day10/src/part2.rs](/day10/src/part2.rs)🎁 | ❄️49.524 ms❄️ | 🎄710.38 ms🎄 |
| 🗓️11🗓️ | ⛄[/day11/src/part1.rs](/day11/src/part1.rs)⛄ | 🎁[/day11/src/part2.rs](/day11/src/part2.rs)🎁 | ❄️69.107 ms❄️ | 🎄324.35 ms🎄 |
| 🗓️12🗓️ | ⛄[/day12/src/part1.rs](/day12/src/part1.rs)⛄ | 🎁[/day12/src/part2.rs](/day12/src/part2.rs)🎁 | ❄️178.45 µs❄️ | 🎄418.57 µs🎄 |
| 🗓️13🗓️ | ⛄[/day13/src/part1.rs](/day13/src/part1.rs)⛄ | 🎁[/day13/src/part2.rs](/day13/src/part2.rs)🎁 | ❄️81.028 ms❄️ | 🎄797.33 ms🎄 |
| 🗓️14🗓️ | ⛄[/day14/src/part1.rs](/day14/src/part1.rs)⛄ | 🎁[/day14/src/part2.rs](/day14/src/part2.rs)🎁 | ❄️1.6078 ms❄️ | 🎄2.5651 ms🎄 |
| 🗓️15🗓️ | ⛄[/day15/src/part1.rs](/day15/src/part1.rs)⛄ | 🎁[/day15/src/part2.rs](/day15/src/part2.rs)🎁 | ❄️13.939 ms❄️ | 🎄15.694 ms🎄 |
| 🗓️16🗓️ | ⛄[/day16/src/part1.rs](/day16/src/part1.rs)⛄ | 🎁[/day16/src/part2.rs](/day16/src/part2.rs)🎁 | ❄️1.8216 ms❄️ | 🎄1.8927 ms🎄 |
| 🗓️17🗓️ | ⛄[/day17/src/part1.rs](/day17/src/part1.rs)⛄ | 🎁[/day17/src/part2.rs](/day17/src/part2.rs)🎁 | ❄️3.0891 µs❄️ | 🎄4.0762 µs🎄 |
| 🗓️18🗓️ | ⛄[/day18/src/part1.rs](/day18/src/part1.rs)⛄ | 🎁[/day18/src/part2.rs](/day18/src/part2.rs)🎁 | ❄️16.952 ms❄️ | 🎄17.450 ms🎄 |
| 🗓️19🗓️ | ⛄[/day19/src/part1.rs](/day19/src/part1.rs)⛄ | 🎁[/day19/src/part2.rs](/day19/src/part2.rs)🎁 | ❄️420.86 µs❄️ | 🎄818.80 µs🎄 |
| 🗓️20🗓️ | ⛄[/day20/src/part1.rs](/day20/src/part1.rs)⛄ | 🎁[/day20/src/part2.rs](/day20/src/part2.rs)🎁 | ❄️340.62 ms❄️ | 🎄364.26 ms🎄 |
| 🗓️21🗓️ | ⛄[/day21/src/part1.rs](/day21/src/part1.rs)⛄ | 🎁[/day21/src/part2.rs](/day21/src/part2.rs)🎁 | ❄️170.99 µs❄️ | 🎄174.96 µs🎄 |
| 🗓️22🗓️ | ⛄[/day22/src/part1.rs](/day22/src/part1.rs)⛄ | 🎁[/day22/src/part2.rs](/day22/src/part2.rs)🎁 | ❄️755.55 ms❄️ | 🎄1.2101 s🎄 |
| 🗓️23🗓️ | ⛄[/day23/src/part1.rs](/day23/src/part1.rs)⛄ | 🎁[/day23/src/part2.rs](/day23/src/part2.rs)🎁 | ❄️17.470 µs❄️ | 🎄11.083 µs🎄 |

## Template

Do you want to use my rust template for your own solutions? 

Then just get the `template` folder from this repository and use the following command to create a new day:

```sh
cargo generate --path ./template
```

Then it will ask you for the project name, just enter the name you want to give for the day and it will create a new folder with the template code for you.

## Author

⛄ [ALXS](https://github.com/ALXS-GitHub)
