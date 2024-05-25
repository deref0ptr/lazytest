# lazytest

Provides a macro which reduces the boilerplate required for simple unit tests.

## Usage

Given the function:

```rust
pub fn answer() -> usize {
    42
}
```

These are equivalent:

```rust
use lazytest::lazytest;

lazytest! {
    check_answer {
        assert_eq!(answer(), 42);
    }
}
```

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_answer() {
        assert_eq!(answer(), 42);
    }
}
```