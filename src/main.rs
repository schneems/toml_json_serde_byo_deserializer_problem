/// Compiles fine:
///
///   $ cargo test --features toml
#[cfg(feature = "toml")]
fn deserializer_toml<'de>(input: &'de str) -> impl serde::Deserializer<'de> {
    toml::Deserializer::new(input)
}

/// Fails with error:
///
///   $ cargo test --features serde_json
///
///   error[E0277]: the trait bound `serde_json::Deserializer<StrRead<'_>>: serde::Deserializer<'de>` is not satisfied
///   --> src/main.rs:7:47
///    |
///7   | fn deserializer_toml<'de>(input: &'de str) -> impl serde::de::Deserializer<'de> {
///    |                                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `serde::Deserializer<'de>` is not implemented for `serde_json::Deserializer<StrRead<'_>>`
///8   |     serde_json::Deserializer::from_str(input)
///    |     ----------------------------------------- return type was inferred to be `serde_json::Deserializer<StrRead<'_>>` here
///    |
///    = help: the trait `Deserializer<'de>` is not implemented for `serde_json::Deserializer<StrRead<'_>>`
///            but trait `Deserializer<'_>` is implemented for `&mut serde_json::Deserializer<_>`
///    = help: for that trait implementation, expected `&mut serde_json::Deserializer<_>`, found `serde_json::Deserializer<StrRead<'_>>`
#[cfg(feature = "serde_json")]
fn deserializer_json<'de>(input: &'de str) -> impl serde::Deserializer<'de> {
    serde_json::Deserializer::from_str(input)
}

fn main() {
    println!("Hello, world!");
}
