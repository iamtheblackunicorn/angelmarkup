/*
Angel Markup Language by Alexander Abraham,
a.k.a. "Angeldust Duke" a.k.a. "The Black Unicorn".
Licensed under the MIT license.
*/

use aml::*;
use std::env;

/// AML tiny-ass CLI.
fn cli() {
    let args: Vec<String> = env::args().collect();
    let arg_len = args.len();
    if arg_len == 5 {
        if args[1].clone() == String::from("-i") &&
        file_is(args[2].clone()) &&
        args[3].clone() == String::from("-o") {
            compile_to_json(args[2].clone(), args[4].clone());
        }
        else {error_out();}
    }
    else {error_out();}
}

/// Main entry point for the Rust
/// compiler.
fn main(){
    cli();
}
