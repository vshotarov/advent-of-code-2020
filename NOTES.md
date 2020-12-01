## Day 01
- Investigate how `use std::io::{self, Read};` works
- Passing values with an `&` in front of them passes them by immutable reference. In order to pass them by a mutable one we do `&mut` in front of them.
- The `?` operator performs basic error handling on the function before it. If it returns `Ok` then it unwraps and gives the inner value. If it's `Err` it returns.
	Ex. `let mut f = File::open("file.txt")?;`
