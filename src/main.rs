/*
Angel Markup Language by Alexander Abraham,
a.k.a. "Angeldust Duke" a.k.a. "The Black Unicorn".
Licensed under the MIT license.
*/

use cleasy::App;
use angelmarkup::*;
use colored::Colorize;

/// AML's tiny-ass CLI.
fn cli() {
    let name: String = String::from("Angelmarkup Language Compiler");
    let version: String = String::from("1.1.0");
    let author: String = String::from("Alexander Abraham");
    let mut aml_cli: App = App::new(name, version, author);
    aml_cli.add_arg("inf".to_string(), "AML input file".to_string(), "true".to_string());
    aml_cli.add_arg("ouf".to_string(), "JSON output file".to_string(), "true".to_string());

    if aml_cli.version_is() == true {
        println!("{}", aml_cli.version().cyan().to_string());
    }
    else if aml_cli.help_is() == true {
        println!("{}", aml_cli.help().cyan().to_string());
    }
    else if aml_cli.arg_was_used("inf".to_string()) == true && aml_cli.arg_was_used("ouf".to_string()) == true {
        let input_file: String = aml_cli.get_arg_data("inf".to_string());
        let output_file: String = aml_cli.get_arg_data("ouf".to_string());
        compile_to_json(input_file,output_file);
    }
    else {
        println!("{}", aml_cli.help().cyan().to_string());
    }
}

/// Main entry point for the Rust
/// compiler.
fn main(){
    cli();
}
