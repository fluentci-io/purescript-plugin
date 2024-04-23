# Purescript Plugin

[![ci](https://github.com/fluentci-io/purescript-plugin/actions/workflows/ci.yml/badge.svg)](https://github.com/fluentci-io/purescript-plugin/actions/workflows/ci.yml)

This plugin sets up your CI/CD pipeline with a specific version of [purescript](https://www.purescript.org/).

## 🚀 Usage

Add the following command to your CI configuration file:

```bash
fluentci run --wasm purescript setup
```

## Functions

| Name          | Description                                                       |
| ------------- | ----------------------------------------------------------------- |
| setup         | Installs a specific version of purescript.                             |
| build         | Install the dependencies and compile the current package |
| test          | Test the project with some module, default Test.Main |
| bundle_app    | Bundle the project into an executable |
| bundle_module | Bundle the project into a module |
| docs          | Generate docs for the project and its dependencies |
| install       | Install (download) all dependencies listed in spago.dhall |
| verify        | Verify that a single package is consistent with the Package Set |
| verify_set   | Verify that the whole Package Set builds correctly |
| bump_version |  Bump and tag a new version, and generate bower.json, in preparation for release. |

## Code Usage

Add `fluentci-pdk` crate to your `Cargo.toml`:

```toml
[dependencies]
fluentci-pdk = "0.1.9"
```

Use the following code to call the plugin:

```rust
use fluentci_pdk::dag;

// ...

dag().call("https://pkg.fluentci.io/purescript@v0.1.0?wasm=1", "setup", vec!["latest"])?;
```

## 📚 Examples

Github Actions:

```yaml
- name: Setup Fluent CI CLI
  uses: fluentci-io/setup-fluentci@v5
  with:
    wasm: true
    plugin: purescript
    args: |
      setup
    working-directory: example
- name: Show purescript version
  run: |
    export PATH=${HOME}/.bun/bin:${PATH}
    type purescript
    type spago
    purescript --version
    spago --version
```
