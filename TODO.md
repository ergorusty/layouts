- [ ] Write Tests
- [ ] [Improve Serde API and Output]
- [ ] [Filtering]
- [ ] [HTML Output]
- [ ] [Coloring]
- [ ] [Replace pretty]

[improve serde api and output]: #improve serde api and output
[filtering]: #filtering
[html output]: #html-output
[coloring]: #coloring
[replace pretty]: #replace-pretty

## Improve Serde API and Output

A major goal of `layouts` is to make it easy to format any Serde-compatible data structure with zero-config.

- [ ] Make this a primary entry point of `layouts`
- [ ] Improve the output

Currently, when formatted on multiple lines, nested data structures are formatted as:

```ts
{
  node: {
    // ...
  }
}
```

This should be:

```ts
{
  node: {
    // ...
  }
}
```

## Filtering

```rust
TychoElement(...).filter(Collapse(&["node.extensions"])).filter(Exclude(&["version_info"]))
```

## HTML Output

- [ ] Direct translation (`pre`)
- [ ] Tree UI
- [ ] Enhanced Tree UI with filtering, etc.

## Coloring

- [ ] Bake colors in at the core `Text` level
- [ ] Wrap pretty's annotations and render APIs
- [ ] Support plain-text rendering
- [ ] Support colors in HTML output

## Replace Pretty

I previously attempted a promising wholesale rewrite of `pretty` on different primitive grounds. Revisit this if `pretty` limitations get too annoying.

TLDR: the way pretty handles indentation is very derived from Wadler's primitives, and results in "hanging" output without some workarounds. Nested data structures are the raison detre of `layouts`, so working off a primitive that considers them from the jump might be useful.

Also, coloring is bolted on using a somewhat quirky "annotations" API, and that API is based on a very particular model of coloring text. It would be better if the concept of "styled text" was the base primitive, and therefore measuring the width of colored text would fall out of the fundamentals.

Neither of these are good enough justifications to prioritize a replacement at this time, but we should keep an eye on it.

The goal of the high-level API is to hide the low-level well enough to be able to support a replacement when needed.
