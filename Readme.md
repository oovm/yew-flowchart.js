Flowchart.js for Yew
====================

Render flow chart with Flowchart.js in Yew!

- The online preview: https://galaster.github.io/yew-flowchart.js

## How to use

1. No CDN needed

2. Easily used by `<FlowChartJS/>`

```rust
use yew_flowchartjs::FlowChartJS;

html! {
    <FlowChartJS code=&self.input/>
}
```

## Todo

- [ ] Chrome only:

```js
Error: <tspan> attribute dy: Expected length, "NaN".
```

Fix: https://github.com/DmitryBaranovskiy/raphael/issues/593

