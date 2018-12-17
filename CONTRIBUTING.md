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

Check out [how to add a crate](add_crate.md) for more details of all the other files that need to be modified.
