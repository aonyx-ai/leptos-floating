# leptos-floating

Floating UI positioning primitives for
[Leptos](https://github.com/leptos-rs/leptos).

Provides reactive positioning for floating elements (tooltips, popovers,
dropdowns, etc.) relative to a reference element, inspired by
[Floating UI](https://floating-ui.com/).

## Usage

```rust
use leptos::prelude::*;
use leptos_floating::*;

#[component]
fn Tooltip() -> impl IntoView {
    let reference_ref = NodeRef::new();
    let floating_ref = NodeRef::new();

    let floating = use_floating(
        reference_ref,
        floating_ref,
        FloatingOptions::builder()
            .side(Side::Bottom)
            .align(Align::Center)
            .side_offset(8.0)
            .build(),
    );
    let x = floating.x();
    let y = floating.y();

    view! {
        <button node_ref=reference_ref>"Hover me"</button>
        <div
            node_ref=floating_ref
            style:position="absolute"
            style:left=move || format!("{}px", x.get())
            style:top=move || format!("{}px", y.get())
        >
            "Tooltip content"
        </div>
    }
}
```

## API

### `use_floating`

Returns a `UseFloatingReturn` with reactive `x`/`y` signals that update when the
reference or floating element changes. Read them with the getters `x()`, `y()`,
`side()`, and `align()`.

### `FloatingOptions`

Build it with `FloatingOptions::builder()`. Read the fields with getters of the
same name.

| Field          | Type    | Default        | Description                         |
| -------------- | ------- | -------------- | ----------------------------------- |
| `side`         | `Side`  | `Side::Bottom` | Which side to place the floating el |
| `align`        | `Align` | `Align::Start` | Alignment along the cross-axis      |
| `side_offset`  | `f64`   | `0.0`          | Distance from the reference element |
| `align_offset` | `f64`   | `0.0`          | Offset along the cross-axis         |

### `Side`

`Top` | `Right` | `Bottom` | `Left`

### `Align`

`Start` | `Center` | `End`

### `calculate_position_from_rect`

Pure function for computing position from bounding rect values — useful for
testing without a DOM.

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or
[MIT License](LICENSE-MIT) at your option.
