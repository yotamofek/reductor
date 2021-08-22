# reductor

Generic abstractions for combining and nesting reduction patterns for iterables.

Docs: [https//docs.rs/reductor](https://docs.rs/reductor)

## Before & After:

### Before

```rust
fn process_samples(samples: &[i32], scale: &[i32], upper_limit: i32) {
    let mut sum = 0;
    let mut min = None;
    let mut max = None;

    for (sample, scale) in samples.iter().zip(scale) {
        let scaled = sample * scale;

        if scaled <= upper_limit {
            continue;
        }

        sum += scaled;
        min = Some(match min {
            Some(min) => scaled.min(min),
            None => scaled,
        });
        max = Some(match max {
            Some(max) => scaled.max(max),
            None => scaled,
        });
    }

    // ...
}
```

### After

```rust
use reductor::{Reduce, ReductorPair, Sum, Min, Max};

fn process_samples(samples: &[i32], scale: &[i32], upper_limit: i32) {
    let ReductorPair(
        Sum::<i32>(sum),
        ReductorPair(Min::<Option<i32>>(min), Max::<Option<i32>>(max)),
    ) = samples
        .iter()
        .zip(scale)
        .map(|(sample, scale)| sample * scale)
        .filter(|&scaled| scaled <= upper_limit)
        .reduce_with();

    // ...
}
```
