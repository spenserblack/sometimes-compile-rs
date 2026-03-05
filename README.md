# Sometimes compile

This crate provides an attribute macro that randomly forgets some tokens. Add this to
your crate and it might compile, or it might not. There is a non-zero chance that it
fixes your code.

## Usage

When a friend you want to prank has walked away from their computer, add this crate
to the dependencies and then annotate something with
`#[sometimes_compile::main]`. See [the example][example]. You can clone this
repository and call [`rerun-until-fail.sh`](./rerun-until-fail.sh) to see it in
action.

You can be extra annoying by renaming the dependency.

```toml
[dependencies]
useful-crate = { package = "sometimes-compile", version = "*" }
```

## Example result

This output was created by trying to compile [an example][example].

```console
error: missing `fn` for function definition
 --> examples/example.rs:2:4
  |
2 | fn main() {
  |    ^^^^
  |
help: add `fn` here to parse `main` as a function
  |
2 | fn fn main() {
  |    ++
```

That's a confusing help message.

## Notes

Each token has a 1/100 chance to be forgotten. The more tokens in the annotated item,
the more likely at least one token will be forgotten.

There is a very small chance that your code will still be in a compilable state after
one or more tokens have been forgotten.

[example]: ./examples/example.rs
