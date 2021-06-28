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

After that you can start fuzzing by doing:

```
cargo afl fuzz -i in -o out target/debug/matrix-rust-sdk-fuzz-target
```

## Learn more

Check out https://rust-fuzz.github.io/book/afl/tutorial.html for some more information