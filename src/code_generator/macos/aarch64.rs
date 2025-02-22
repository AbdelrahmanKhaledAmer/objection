use super::super::super::parser::ast::*;

use crate::code_generator::CodeGenerator;

pub struct MacOsAarch64;

impl CodeGenerator for MacOsAarch64 {
    fn generate(&self, prog: &NodeProg) -> String {
        let mut lines = vec![
            ".global _main".to_string(),
            "_main:".to_string(),
            "    bl objection_main".to_string(),
            "    mov x16, #1".to_string(),
            "    svc #0x80".to_string(),
        ];
        for function in prog.functions.iter() {
            self.generate_function(&function, &mut lines);
        }
        lines.join("\n")
    }
}

impl MacOsAarch64 {
    fn generate_function(&self, func: &NodeFunc, lines: &mut Vec<String>) {
        let func_name = format!("objection_{}", func.ident.name);
        let func_global = format!(".global {}", func_name);
        let func_header = format!("{}:", func_name);
        lines.push(func_global);
        lines.push(func_header);
        // STP (store pair) of x29 and x30 to the stack. Each register is 8 bytes.
        lines.push("    stp x29, x30, [sp, #-16]!".to_string());
        // Set the frame pointer to the current stack pointer.
        lines.push("    mov x29, sp".to_string());
        // This is where I should allocate stack space for parameters.
        // Finally, generate the function body (block).
        self.generate_block(&func.block, lines);
    }

    fn generate_block(&self, block: &NodeBlock, lines: &mut Vec<String>) {
        for stmt in block.stmts.iter() {
            self.generate_stmt(&stmt, lines);
        }
    }

    fn generate_stmt(&self, stmt: &NodeStmt, lines: &mut Vec<String>) {
        match stmt {
            NodeStmt::Return(expr) => {
                self.generate_expr(&expr, lines);
                // Load the value in x9 (the first temporary register which will be used to store
                // expression results) into x0 (the return register).
                lines.push("    mov x0, x9".to_string());
                // LDP (load pair) of x29 and x30 from the stack. Each register is 8 bytes.
                lines.push("    ldp x29, x30, [sp], #16".to_string());
                // Return from the function.
                lines.push("    ret".to_string());
            }
        }
    }

    fn generate_expr(&self, expr: &NodeExpr, lines: &mut Vec<String>) {
        match expr {
            NodeExpr::Literal(val) => {
                // Load the integer literal into x9 (the first temporary register)
                match val {
                    NodeLiteral::IntLit(val) => {
                        lines.push(format!("    mov x9, #{}", val));
                    }
                }
            }
        }
    }
}
