## Contributing
Crate Race will only benchmark crates that share functionality with other crates.

If you have such shared crate functions, please feel free to add in a Pull Request with the extra crate.

Please adhere to the standard visible with all other benchmarks in this project:

Only one of the crates functions should be tested per bench folder.

Name it "_[cratename].rs" inside the folder.

Include this template code in there:

    use bencher::Bencher;

    pub fn baseline(b: &mut Bencher) {
      b.iter(|| {
        //Run baseline crate function here
        //Add assert_eq!() code here to test the output of the function
      });
    }

If there are multiple tests for the function, feel free to add in these tests into this file.

## Testing your new create

Update `crate-race/benches/*/bench.rs`:

1. add `mod _[crate];`
2. Update any listed `benchmark_group!` and `benchmark_main!` to include `[crate]::function`

In the terminal, type in:

    cargo bench --bench [bench_name]

Bench name is the name of the folder where you saved the file.

## Making crate-race pick up your new crate

1. Make a file `crate-race/benches/*/_[crate].rs` with benchmarks specific to that crate.
2. Update `crate-race/benches/*/bench.rs`:
  a. add `mod _[crate];`
  b. Update any listed `benchmark_group!` and `benchmark_main!` to include `[crate]::function`
3. Update `crate-race/Cargo.toml`, with `[crate] = "*"` close to other crates of the same bench.
4. Update `crate_list.csv`, with [crate],0,[bench1],[bench2],[bench3],etc. Order the crate alphabetically.

In the terminal, type in:

    cargo run --release

Crate-Race will detect changes based on crate versions differing from what's in `crate_list.csv`

In step #4 above, you set the version to 0, so that will force crate-race into looking at your crate.

## Discussion / Suggestions
Go here for any issues / discussions / suggestions:

https://www.reddit.com/r/crate_race/
