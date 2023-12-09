<img src="./.assets/christmas_ferris.png" width="164">

# 🎄 Advent of Code 2023

Solutions for [Advent of Code](https://adventofcode.com/) in [Rust](https://www.rust-lang.org/).

<!--- advent_readme_stars table --->
## 2023 Results

| Day | Part 1 | Part 2 |
| :---: | :---: | :---: |
| [Day 1](https://adventofcode.com/2023/day/1) | ⭐ | ⭐ |
<!--- advent_readme_stars table --->

<!--- benchmarking table --->

---

## Usage

### Scaffold a day

```sh
# example: `cargo scaffold 1`
cargo scaffold <day>

# output:
# Created module file "src/bin/01.rs"
# Created empty input file "data/inputs/01.txt"
# Created empty example file "data/examples/01.txt"
# ---
# 🎄 Type `cargo solve 01` to run your solution.
```

Individual solutions live in the `./src/bin/` directory as separate binaries. _Inputs_ and _examples_ live in the the `./data` directory.

Every [solution](https://github.com/fspoettel/advent-of-code-rust/blob/main/src/template.txt) has _tests_ referencing its _example_ file in `./data/examples`. Use these tests to develop and debug your solutions against the example input. In VS Code, `rust-analyzer` will display buttons for running / debugging these unit tests above the unit test blocks.

> [!TIP]
> If a day has different example inputs for both parts, you can use the `read_file_part()` helper in your tests instead of `read_file()`. For example, if this applies to day 1, you can create a second example file `01-2.txt` and invoke the helper like `let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));` to read it in `test_part_two`.

### Download input & description for a day

> [!IMPORTANT] 
> This requires [installing the aoc-cli crate](#configure-aoc-cli-integration).

You can automatically download puzzle inputs and description by either appending the `--download` flag to `scaffold` (e.g. `cargo scaffold 4 --download`) or with the separate `download` command:

```sh
# example: `cargo download 1`
cargo download <day>

# output:
# [INFO  aoc] 🎄 aoc-cli - Advent of Code command-line tool
# [INFO  aoc_client] 🎅 Saved puzzle to 'data/puzzles/01.md'
# [INFO  aoc_client] 🎅 Saved input to 'data/inputs/01.txt'
# ---
# 🎄 Successfully wrote input to "data/inputs/01.txt".
# 🎄 Successfully wrote puzzle to "data/puzzles/01.md".
```

### Run solutions for a day

```sh
# example: `cargo solve 01`
cargo solve <day>

# output:
#     Finished dev [unoptimized + debuginfo] target(s) in 0.13s
#     Running `target/debug/01`
# Part 1: 42 (166.0ns)
# Part 2: 42 (41.0ns)
```

The `solve` command runs your solution against real puzzle inputs. To run an optimized build of your code, append the `--release` flag as with any other rust program.

By default, `solve` executes your code once and shows the execution time. If you append the `--time` flag to the command, the runner will run your code between `10` and `10.000` times (depending on execution time of first execution) and print the average execution time.

For example, running a benchmarked, optimized execution of day 1 would look like `cargo solve 1 --release --time`. Displayed _timings_ show the raw execution time of your solution without overhead like file reads.

#### Submitting solutions

> [!IMPORTANT]
> This requires [installing the aoc-cli crate](#configure-aoc-cli-integration).

In order to submit part of a solution for checking, append the `--submit <part>` option to the `solve` command.

### Run all solutions

```sh
cargo all

# output:
#     Running `target/release/advent_of_code`
# ----------
# | Day 01 |
# ----------
# Part 1: 42 (19.0ns)
# Part 2: 42 (19.0ns)
# <...other days...>
# Total: 0.20ms
```

This runs all solutions sequentially and prints output to the command-line. Same as for the `solve` command, the `--release` flag runs an optimized build.

#### Update readme benchmarks

The template can output a table with solution times to your readme. In order to generate a benchmarking table, run `cargo time`. If everything goes well, the command will output "_Successfully updated README with benchmarks._" after the execution finishes and the readme will be updated.

Please note that these are not "scientific" benchmarks, understand them as a fun approximation. 😉 Timings, especially in the microseconds range, might change a bit between invocations.

### Run all tests

```sh
cargo test
```

To run tests for a specific day, append `--bin <day>`, e.g. `cargo test --bin 01`. You can further scope it down to a specific part, e.g. `cargo test --bin 01 part_one`.

### Format code

```sh
cargo fmt
```

### Lint code

```sh
cargo clippy
```

### Read puzzle description in terminal

> [!IMPORTANT]
> This command requires [installing the aoc-cli crate](#configure-aoc-cli-integration).

```sh
# example: `cargo read 1`
cargo read <day>

# output:
# Loaded session cookie from "/Users/<snip>/.adventofcode.session".
# Fetching puzzle for day 1, 2022...
# ...the input...
```
