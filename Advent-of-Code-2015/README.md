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

## Template

Do you want to use my rust template for your own solutions? 

Then just get the `template` folder from this repository and use the following command to create a new day:

```sh
cargo generate --path ./template
```

Then it will ask you for the project name, just enter the name you want to give for the day and it will create a new folder with the template code for you.

## Author

⛄ [ALXS](https://github.com/ALXS-GitHub)