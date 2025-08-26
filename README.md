# Crypto Swiss Knife

TUI app for exploring classical and modern cryptography. Code is split into:

- `src/algorithms/`: Pure crypto logic (no UI). Small, testable functions.
- `src/components/`: TUI widgets implementing the `CipherComponent` trait.
- `src/tabs/`: High-level tabs that group components and handle navigation.
- `src/main.rs`: App shell, tab switching, and global layout.

## Getting started

Run the app:

```bash
cargo run
```

## Where to start if you want to add a new cipher

1) Implement the algorithm (start here)
- Copy `src/algorithms/TEMPLATE_algorithm.rs` to `src/algorithms/my_cipher.rs`.
- Implement your functions (e.g., `my_cipher_encrypt`, `my_cipher_decrypt`).
- Export it in `src/algorithms/mod.rs` by adding:
  ```rust
  pub mod my_cipher;
  ```
- See `src/algorithms/README.md` for details.

2) Build the UI component
- Copy `src/components/TEMPLATE_component.rs` to `src/components/my_cipher.rs`.
- In your component’s `Enter` handler, call your algorithm functions.
- Export it in `src/components/mod.rs` by adding:
  ```rust
  pub mod my_cipher;
  ```
- See `src/components/README.md` for the trait and template.

3) Add the component to a tab
- If it’s a classical cipher, add to `src/tabs/classical.rs`; if symmetric, add to `src/tabs/symmetric.rs`.
- Add your component to the `components: Vec<Box<dyn CipherComponent>>` in `Default::default()`.
- See `src/tabs/README.md` for examples.

4) (Optional) Create a new tab category
- Copy `src/tabs/TEMPLATE_tab.rs` to `src/tabs/my_category.rs` and export it in `src/tabs/mod.rs`.
- Wire it in `src/main.rs` by adding a new `CryptoTab` variant, a field on `App`, and handling it in `render(...)` and event methods.
- Steps are outlined in `src/tabs/README.md`.

## Code conventions
- Keep crypto logic in `algorithms` and UI state/rendering in `components`/`tabs`.
- Prefer `Result<T, anyhow::Error>` for fallible functions.
- Do not perform file I/O in components unless strictly necessary; prefer passing strings/bytes.

## Templates & docs
- Algorithms: `src/algorithms/TEMPLATE_algorithm.rs`, `src/algorithms/README.md`
- Components: `src/components/TEMPLATE_component.rs`, `src/components/README.md`
- Tabs: `src/tabs/TEMPLATE_tab.rs`, `src/tabs/README.md`
