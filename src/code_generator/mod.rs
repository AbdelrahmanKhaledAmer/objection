use std::env::consts::ARCH;
use std::env::consts::OS;
use std::env::var;

use super::parser::ast::*;
mod macos {
    pub mod aarch64;
}

pub trait CodeGenerator {
    fn generate(&self, prog: &NodeProg) -> String;
}

pub fn code_generator_factory() -> Box<dyn CodeGenerator> {
    let os = var("TARGET_OS").unwrap_or(OS.to_string());
    let arch = var("TARGET_ARCH").unwrap_or(ARCH.to_string());
    match os.as_str() {
        "macos" => match arch.as_str() {
            "aarch64" => Box::new(macos::aarch64::MacOsAarch64),
            _ => panic!("UNSUPPORTED ARCHITECTURE: {}", arch),
        },
        _ => panic!("UNSUPPORTED OPERATING SYSTEM: {}", os),
    }
}
