use std::env::args;
use std::fs::read_to_string;

mod lexer {
    pub mod lexer;
    mod tokens;
}

fn main() {
    // The command to run this compiler will have the following format:
    //     $ cargo run -- file_name
    // I will also add some options to help us with debugging
    //     $ cargo run -- file_name -t --ast --asm
    // The first argument is the name of the file to compile
    // The second argument is the option to print the tokens generated by the lexer
    // The third argument is the option to print the AST generated by the parser
    // The fourth argument is the option to print the assembly code generated by the code generator
    // All the commands are optional except for the file name, and they can be in any order

    // Firstly, we will get the arguments passed to the program
    let args: Vec<String> = args().collect();
    let mut file_name = "";
    let mut print_tokens = false;
    // let mut print_ast = false;
    // let mut print_asm = false;
    for arg in args.iter().skip(1) {
        if arg == "-t" {
            print_tokens = true;
        } else if arg == "--ast" {
            // print_ast = true;
        } else if arg == "--asm" {
            // print_asm = true;
        } else {
            if file_name != "" {
                panic!("Multiple file names provided, {} and {}", file_name, arg);
            }
            file_name = arg;
        }
    }
    if file_name == "" {
        panic!("No file name provided");
    }

    // Now we will read the file and pass it to the lexer
    let source_code =
        read_to_string(file_name).expect(format!("Could not read file {}", file_name).as_str());
    let tokens = lexer::lexer::get_tokens(source_code);
    // Print the tokens if the option is enabled
    if print_tokens {
        println!("{:?}", tokens);
    }
}
