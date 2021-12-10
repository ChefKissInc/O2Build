use std::fmt::Write;

use crate::ast::{CompilationUnit, Node};

pub trait CompUnitToAsm {
    fn to_llvm_ir(&self) -> String;
}

impl CompUnitToAsm for CompilationUnit {
    fn to_llvm_ir(&self) -> String {
        // let mut ret = String::new();

        // for member in &self.members {
        //     match member {
        //         Node::FunctionDefinition(func_def) => {
        //             if func_def.public {
        //                 writeln!(ret, "global {}", func_def.symbol).unwrap();
        //             }
        //             write!(ret, "{}:\n\t", func_def.symbol).unwrap();

        //             for _node in &func_def.body {}

        //             writeln!(ret, "ret ; End of {}", func_def.symbol).unwrap();
        //         }
        //         Node::StaticDecl => todo!(),
        //         _ => panic!("This shouldn't be here"),
        //     }
        // }

        // ret
        todo!()
    }
}
