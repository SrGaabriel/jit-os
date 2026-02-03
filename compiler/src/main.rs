#![no_std]
#![no_main]

use raph_api::{io::fs::MappedFile, println};

use crate::syntax::{SourceFile, lexer::Lexer};

extern crate raph_common;
extern crate raph_runtime;

pub mod syntax;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> i32 {
    println!("Hello, World!");

    if let Some(file) = MappedFile::open("test_program") {
        println!("Read {} bytes from file:", file.len());
        let source_file = SourceFile {
            id: 0,
            name: "test_program",
            source: file.as_bytes(),
            package: None,
        };
        let lexer = Lexer::new(&source_file);
        println!("Tokens:");
        for result in lexer {
            match result {
                Ok(token) => println!("{:?}", token),
                Err(err) => println!("Error: {:?}", err),
            }
        }
        return 0;
    } else {
        println!("File not found!");
        return 1;
    }
}
