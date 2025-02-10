# Understanding serde::Deserializer

## What

I want to write an interface that allows people to BYO Deserializer. If I understand correctly, [serde::Deserializer](https://docs.rs/serde/1.0.217/serde/trait.Deserializer.html) is the trait to use for bounds.

I'm able to get this to work with the `toml` crate, but can't figure out how to get it to work with other crates, like `serde_json`. This repo demonstrates that problem. I want:

- To solve my original, high level goal.
- And, to understand the serde_json code better. I am having a hard time understnading where some constraints and functionality are coming from.

## Expected

I'm able to compile `toml::Deserializer` and `serde_json::Deserializer` to return ` -> impl serde::Deserializer<'de>` and it compiles.

## Actual

Toml works:

```
$ cargo test --features toml
$ echo $?
0
```

Json does not:

```
$ cargo test --features serde_json
   Compiling compare_deser v0.1.0 (/private/tmp/5a1a55a4152d1586aa39e7f88b587c6b/compare_deser)
error[E0277]: the trait bound `serde_json::Deserializer<StrRead<'_>>: serde::Deserializer<'de>` is not satisfied
  --> src/main.rs:25:47
   |
25 | fn deserializer_toml<'de>(input: &'de str) -> impl serde::Deserializer<'de> {
   |                                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `serde::Deserializer<'de>` is not implemented for `serde_json::Deserializer<StrRead<'_>>`
26 |     serde_json::Deserializer::from_str(input)
   |     ----------------------------------------- return type was inferred to be `serde_json::Deserializer<StrRead<'_>>` here
   |
   = help: the trait `Deserializer<'de>` is not implemented for `serde_json::Deserializer<StrRead<'_>>`
           but trait `Deserializer<'_>` is implemented for `&mut serde_json::Deserializer<_>`
   = help: for that trait implementation, expected `&mut serde_json::Deserializer<_>`, found `serde_json::Deserializer<StrRead<'_>>`

For more information about this error, try `rustc --explain E0277`.
error: could not compile `compare_deser` (bin "compare_deser" test) due to 1 previous error
```

## Questions

From the output I can see that the trait is implemented on `&mut serde_json::Deserializer` but not on `serde_json::Deserializer`.

- I don't understand what in the `serde_json` crate is forcing this distinction.
- I don't understand why the toml crate works but the serde_json one doesn't.

Toml:

```
#[cfg(feature = "parse")]
pub struct Deserializer<'a> {
    input: &'a str,
}

#[cfg(feature = "parse")]
impl<'a> Deserializer<'a> {
    /// Deserialization implementation for TOML.
    pub fn new(input: &'a str) -> Self {
        Self { input }
    }
}
```

Toml can deserialize without needing to be mutable:

```
de::Deserialize::deserialize(de)
```

With serde_json it doesn't like like `serde::Deserializer<'de>` is implemented for `serde_json::Deserializer` but rather it implements Deserializer on concrete types:

```
impl<'de> serde::Deserializer<'de> for Map<String, Value>
```

## More info

Ultimately I want an interface that allows the user to BYO a deserializer. The code currently something like this:

```rust
trait ByoDeserializer
    fn deserializer<'de>(input: &str) -> impl serde::de::Deserializer<'de>;

    #[must_use]
    fn deserialize_from(input: &str) -> Option<Self> {
        if let Ok(instance) = Self::deserialize(Self::deserializer(input)) {
            todo!()
        } else {
            todo!()
        }
    }
    //...
}
```

I could change the trait interface to something much more generic like this:

```rust
fn deserialize_from(input: &str) -> Option<Self>;
```

But it feels like I should be able to say "give me something that can deserialize something else and that should be enough of an interface. Unfortunately I can't return `&mut` from the interface because it would become instantly gone.
