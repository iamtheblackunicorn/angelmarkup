# **A**NGEL **M**ARKUP **L**ANGUAGE :performing_arts: :guitar:

***Simple data storage with no drama.*** :performing_arts: :guitar:

![GitHub CI](https://github.com/iamtheblackunicorn/aml/actions/workflows/rust.yml/badge.svg)

## ABOUT :books:

Since I am currently working on the static-site generator oriented towards blogs called ***Acid***, I need a data-storage format which allows comments and doesn't like drama. **A**NGEL **M**ARKUP **L**ANGUAGE is that format.

## INSTALLATION :inbox_tray:

You should have the following tools installed and available from the command line:

- Rust
- Git

To install ***AML***, simply run this command from a terminal window:

```bash
$ cargo install --git https://github.com/iamtheblackunicorn/aml
```

This should make the `aml` binary available from the command line.

## USAGE :hammer:

### COMMAND LINE

Assuming the installation worked for you, you can now compile ***AML*** files to ***JSON*** files using this command:

```bash
$ aml -i sample.aml -o sample.json
```

`sample.aml` is fed in, `sample.json` is fed out. `sample.aml` might look something like this:

```text
'name' => 'aml'
// This is a comment.
'version' => '1.0.0'
```

### AS A RUST DEVELOPER

***AML*** is mainly oriented towards developers. If you would like to use the tool's functions, please look at `src/lib.rs`.
To add ***AML's*** library to your Rust project, add this line to your project's `Cargo.toml`:

```TOML
aml = { git = "https://github.com/iamtheblackunicorn/aml", version = "1.0.0" }
```

To use ***AML's*** library in your Rust code, add this line to wherever you want to use ***AML's*** functions:

```Rust
use aml::*;
```

## CHANGELOG :black_nib:

### Version 1.0.0

- Initial release.
- Upload to GitHub.

## NOTE :scroll:

- ***A**NGEL **M**ARKUP **L**ANGUAGE :performing_arts: :guitar:* by Alexander Abraham :black_heart: a.k.a. *"The Black Unicorn" :unicorn:*
- Licensed under the MIT license.
