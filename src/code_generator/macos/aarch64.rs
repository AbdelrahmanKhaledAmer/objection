use std::collections::HashMap;

use super::super::super::parser::ast::*;

use crate::code_generator::CodeGenerator;

pub struct MacOsAarch64 {
    scope_offsets_stack: Vec<HashMap<String, i32>>,
    current_offset: i32,
}

impl CodeGenerator for MacOsAarch64 {
    fn generate(&mut self, prog: &NodeProg) -> String {
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
    pub fn new() -> Self {
        MacOsAarch64 {
            scope_offsets_stack: Vec::new(),
            current_offset: 0,
        }
    }

    fn generate_function(&mut self, func: &NodeFunc, lines: &mut Vec<String>) {
        let func_name = format!("objection_{}", func.ident.name);
        let func_global = format!(".global {}", func_name);
        let func_header = format!("{}:", func_name);
        lines.push(func_global);
        lines.push(func_header);
        // STP (store pair) of x29 and x30 to the stack. Each register is 8 bytes.
        lines.push("    stp x29, x30, [sp, #-16]!".to_string());
        // Set the frame pointer to the current stack pointer.
        lines.push("    mov x29, sp".to_string());
        // Reset the current offset to 0.
        self.current_offset = 0;
        self.scope_offsets_stack.push(HashMap::new());
        // This is where I should allocate stack space for parameters.
        // Finally, generate the function body (block).
        self.generate_block(&func.block, lines);
        self.scope_offsets_stack.pop();
    }

    fn generate_block(&mut self, block: &NodeBlock, lines: &mut Vec<String>) {
        self.scope_offsets_stack.push(HashMap::new());
        for stmt in block.stmts.iter() {
            self.generate_stmt(&stmt, lines);
        }
        self.scope_offsets_stack.pop();
    }

    fn generate_stmt(&mut self, stmt: &NodeStmt, lines: &mut Vec<String>) {
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
            NodeStmt::Assign(ident, _, expr) => {
                self.generate_expr(&expr, lines);
                // Store the value in x9 (which contains expression result) onto the stack.
                // For now, all vars are 64-bit integers, so we store 8 bytes.
                self.current_offset -= 8;
                let offset = self.current_offset;
                self.scope_offsets_stack
                    .last_mut()
                    .expect("Error, no valid scope found")
                    .insert(ident.name.clone(), offset);
                lines.push(format!("    str x9, [sp, #{}]", offset));
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
            NodeExpr::Ident(ident) => {
                // Load the variable value into x9 (the first temporary register)
                let offset = self
                    .scope_offsets_stack
                    .iter()
                    .rev()
                    .find_map(|scope_map| scope_map.get(&ident.name))
                    .expect(format!("Error, variable {} not found", ident.name).as_str());
                lines.push(format!("    ldr x9, [sp, #{}]", offset));
            }
        }
    }
}
