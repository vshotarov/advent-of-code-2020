## Advent of Code 2020 solutions in Rust by a complete beginner
Since, I had done absolutely no Rust programming before picking up day01, a lot of the solutions will probably look horrible to seasoned Rustaceans, but I would hope that as the days progress the code will be getting better.

I've been using [BurntSushi's 2018 AoC repo](https://github.com/BurntSushi/advent-of-code) as a reference for how to structure the project, read the input and run the programs. It was recommended multiple times as a great reference for idiomatic Rust code, so I thought it would be a good idea to steal as much as I can from there.

So, using the instructions part of their README, to run a solution, `cd` into its directory and invoke the program with Cargo:
```
$ cd day01
$ cargo run --release < input/input.txt
```
**NOTE**: You will have to replace `input/input.txt` with your own input data. I've not included my own inputs, as the creator of AoC would prefer not sharing them, according to [this comment on reddit](https://www.reddit.com/r/adventofcode/comments/k99rod/sharing_input_data_were_we_requested_not_to/gf2ukkf/?utm_source=reddit&utm_medium=web2x&context=3)
