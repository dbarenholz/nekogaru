# nekogaru

Easily solve kattis problems.

## Usage

Because competitive programming -- and thus kattis -- is about speed, the usage of nekogaru is somewhat contrived.

To run the current problem that is being worked on:

```bash
cargo t # aliased in .cargo/config.toml to point to the right rs file
```

To start working on a new problem (or switch to an already implemented one):

```bash
cargo r -- <problem-name-or-url>
```

## Inner workings

When you start working on a new problem, we do the following:

1. Create a file `src/problem-name.rs` from `template.rs` (note: not in `src` directory so it doesn't get buried). If it already exists, go directly to step 5.
2. Fetch the kattis page. Either the provided URL, or the problem-name prepended with `https://open.kattis.com/problems/`.
3. Scrape the inputs and outputs out of the page, and modify `src/problem-name.rs` to contain each example as a test.
4. Add a `[[bin]]` section to `Cargo.toml`, setting name to `problem-name` and its path to `src/problem-name.rs`.
5. Modify `.cargo/config.toml` to have `t = test --bin problem-name` as alias.

## Template

The provided template is a starting point for your kattis shenanigans. It contains a rudimentary `Scanner` that allows you to write nice things, as demonstrated below:

```rs
    let inp_str = "(1,2) (3,4)\n(5,6)";
    let inp = std::io::Cursor::new(inp_str);
    let mut scanner = super::Scanner::new(inp);

    let points = scanner.read_array::<Point, 3>();
```

As long as your `Point` implements `FromStr`, this _should_ just work out of the box.
