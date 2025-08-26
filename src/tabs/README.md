# Tabs

High-level navigation aggregating cipher components.

## Add a component to an existing tab
1. Export your component in `components/mod.rs` with `pub mod my_cipher;`.
2. In the tab file (e.g., `tabs/classical.rs`), add it to the `components` vec in `Default::default()`:

```rust
components: vec![
    Box::new(crate::components::caesar::CaesarCipherComponent::default()),
    Box::new(crate::components::my_cipher::MyCipherComponent::default()),
]
```

No other changes needed if your component implements `CipherComponent`.

## Create a new tab category
1. Copy `tabs/TEMPLATE_tab.rs` to `tabs/my_category.rs` and adjust names.
2. Export it from `tabs/mod.rs`:

```rust
pub mod my_category;
```

3. Wire it in `main.rs`:
   - Add a new variant in `CryptoTab`.
   - Add a field in `App` for the new tab's state.
   - Handle it in `render(...)` and `handle_*_events(...)` similar to existing tabs.
