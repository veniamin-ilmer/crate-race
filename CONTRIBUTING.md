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

## Adding a crate

1. Make a file `crate-race/benches/*/_[crate].rs` with benchmarks specific to that crate.
2. Update `crate-race/benches/*/bench.rs`:
  a. add `mod _[crate];`
  b. Update any listed `benchmark_group!` and `benchmark_main!` to include `[crate]::function`
3. Update `crate-race/Cargo.toml`, with `[crate] = "*"` close to other crates of the same bench.
4. Update `crate_list.csv`, with [crate],0,[bench1],[bench2],[bench3],etc. Order the crate alphabetically.
