# assets/

This folder is copied verbatim to `dist/assets/` on every build (see the
`<link data-trunk rel="copy-dir" href="./assets" />` directive in
`index.html`). Reference files from Rust with an absolute path:

```rust
poster: Some("/assets/otelmind-cost-analytics.png"),
```

## Files expected

- `otelmind-cost-analytics.png` — banner for the OTELMIND project card.
  Save the Cost Analytics dashboard screenshot here.
