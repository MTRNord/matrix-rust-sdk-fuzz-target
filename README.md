# Matrix-sdk Fuzzing

## How to run

First you need to install the required binary:

```
cargo install afl
```

After that the binary needs to be compiled. AFL needs extra flags so you should do it this way:

```
cargo afl build
```

In case you want to run another fuzz target add `--bin <target binary name>` to compile it.

After that you can start fuzzing by doing:

```
cargo afl fuzz -i in/main -o out target/debug/matrix-rust-sdk-fuzz-target
```

To run other fuzzers make sure to change the in accordingly and use a different out folder. Also remember to use the correct binary.

## Learn more

Check out https://rust-fuzz.github.io/book/afl/tutorial.html for some more information