# Async Button

Async button handling crate for `no_std` environments. Build around `embedded-hal 1.0` traits and `embassy-time`.

## Example

```rust,ignore
let pin = /* Input pin */;
let mut button = Button::new(pin, ButtonConfig::default());

// In a separate task:
loop {
    match button.update().await {
        ButtonEvent::ShortPress { count } => {/* Do something with short presses */},
        ButtonEvent::LongPress => {/* Do something with long press */},
    }
}
```

## Features

- `defmt`: derives `defmt::Format` on [`ButtonConfig`] and [`ButtonEvent`].
