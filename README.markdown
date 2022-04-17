# **A**NGEL **M**ARKUP **L**ANGUAGE

***Simple data storage with no drama.***

![GitHub CI](https://github.com/iamtheblackunicorn/angelmarkup/actions/workflows/rust.yml/badge.svg)

## ABOUT

Since I am currently working on the static-site generator oriented towards blogs called ***Acid***, I need a data-storage format which allows comments and doesn't like drama. **A**NGEL **M**ARKUP **L**ANGUAGE is that format.

## INSTALLATION

You should have the following tools installed and available from the command line:

- Rust
- Git

To install ***Angel Markup***, simply run this command from a terminal window:

```bash
$ cargo install --git https://github.com/iamtheblackunicorn/angelmarkup
```

This should make the `angelmarkup` binary available from the command line.

## USAGE

### COMMAND LINE

Assuming the installation worked for you, you can now compile ***Angel Markup*** files to ***JSON*** files using this command:

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

***Angel Markup*** is mainly oriented towards developers. If you would like to use the tool's functions, please look at `src/lib.rs`.
To add ***Angel Markup's*** library to your Rust project, add this line to your project's `Cargo.toml`:

```TOML
angelmarkup = { git = "https://github.com/iamtheblackunicorn/angelmarkup", version = "1.0.0" }
```

To use ***Angel Markup's*** library in your Rust code, add this line to wherever you want to use ***Angel Markup's*** functions:

```Rust
use angelmarkup::*;
```

## CHANGELOG

### Version 1.0.0

- Initial release.
- Upload to GitHub.

## NOTE :scroll:

- ***A**NGEL **M**ARKUP **L**ANGUAGE* by Alexander Abraham a.k.a. *"The Black Unicorn"*
- Licensed under the MIT license.
