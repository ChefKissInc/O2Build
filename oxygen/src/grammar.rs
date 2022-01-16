/*
 * Copyright (c) VisualDevelopment 2021-2021.
 * This project is licensed by the Creative Commons Attribution-NoCommercial-NoDerivatives licence.
 */

use crate::{
    abi::CallConv,
    ast::{
        binary::BinaryOp, expression::Expression, function::FunctionPrototype, typing::Type, Node,
        SyntaxTree,
    },
};

peg::parser! {
    pub grammar syntax_tree() for str {
        rule comment() = ("//" [^ '\n']*) / ("/*" (!"*/" [_])+ "*/")
        rule _ = quiet!{([' ' | '\n'] / comment())*} // Optional whitespace
        rule __ = quiet!{([' ' | '\n'] / comment())+} // Mandatory whitespace

        rule ident() -> &'input str = s:(quiet!{$(['a'..='z' | 'A'..='Z']['a'..='z' | 'A'..='Z' | '0'..='9']*)} / expected!("identifier")) { s }
        rule string() -> &'input str = "\"" chars:$([^ '"']*) "\"" { chars }
        rule type_() -> Type = ident:ident() {?
            match ident {
                "Void" => Ok(Type::Void),
                "Int" => Ok(Type::Int),
                "Str" => Ok(Type::Str),
                _ => Err("type \"Void\", \"Int\" or \"Str\""),
            }
        }
        rule ident_with_type() -> Node = ident:ident() _ ":" _ ty:type_() {
            Node::FunctionArgument(ident.to_string(), ty)
        }
        rule ret_type() -> Type = "->" _ ret_type:type_() { ret_type }

        // Function-related
        rule args<T>(arg_match: rule<T>) -> Vec<T> = _ "(" _ args:(arg_match() ** (_ "," _)) _ ","? _ ")" _ { args }
        rule call_conv() -> CallConv = "callconv" __ string:string() __ {?
            match string {
                "C" => Ok(CallConv::C),
                "SystemV" => Ok(CallConv::SystemV),
                "UEFI" => Ok(CallConv::UEFI),
                _ => Err("calling convention \"C\", \"SystemV\" or \"UEFI\""),
            }
        }
        rule func_proto<T>(external: rule<T>) -> FunctionPrototype
                    = public:("public" __)? external() call_conv:call_conv()?
                     "func" __ symbol:ident() _ args:args(<ident_with_type()>) _ ret_type:ret_type()? {
            FunctionPrototype {
                public: public.is_some(),
                symbol: symbol.to_string(),
                call_conv: call_conv.unwrap_or(CallConv::SystemV),
                args,
                ret_type: ret_type.unwrap_or(Type::Void),
            }
        }

        // Expressions
        rule num_expr() -> Expression = _ n:(quiet!{$(['0'..='9']+)} / expected!("number")) _ {
            Expression::IntegerLiteral(n.to_string())
        }
        rule str_expr() -> Expression = _ s:(string() / expected!("string")) _ { Expression::StringLiteral(s.to_string()) }
        rule call_expr() -> Expression = _ name:ident() _ args:args(<expr()>) _ {
            Expression::FunctionCall { name: name.to_string(), args }
        }
        rule parenthesised_expr() -> Expression = "(" _ expr:expr() _ ")" { expr }
        rule binary_expr() -> Expression = _ expr:(
            precedence!{
                x:(@) _ "+" _ y:@ {
                    Expression::Binary {
                        op: BinaryOp::Addition,
                        left_expr: Box::new(x),
                        right_expr: Box::new(y),
                    }
                }
                x:(@) _ "-" _ y:@ {
                    Expression::Binary {
                        op: BinaryOp::Subtraction,
                        left_expr: Box::new(x),
                        right_expr: Box::new(y),
                    }
                }
                --
                x:(@) _ "*" _ y:@ {
                    Expression::Binary {
                        op: BinaryOp::Multiplication,
                        left_expr: Box::new(x),
                        right_expr: Box::new(y),
                    }
                }
                x:(@) _ "/" _ y:@ {
                    Expression::Binary {
                        op: BinaryOp::Division,
                        left_expr: Box::new(x),
                        right_expr: Box::new(y),
                    }
                }
                --
                expr:(num_expr() / call_expr() / parenthesised_expr() / expected!("expression")) { expr }
            } / expected!("binary expression")
        ) _ { expr }
        rule block_expr() -> Expression = _ "{" _ exprs:(expr() ** (_ ";" _)) _ ";"? _ "}" _ { Expression::Block(exprs) }
        rule expr() -> Expression
                    = _ expr:(
                        block_expr()
                        / parenthesised_expr()
                        / binary_expr()
                        / num_expr()
                        / str_expr()
                        / call_expr()
                        / expected!("expression")
                    ) _ { expr }

        // Global members
        rule extern_func() -> Node = _ proto:func_proto(<("external" __)>) _ ";" _ {
            Node::ExternalFunction(proto)
        }
        rule func() -> Node = _ proto:func_proto(<>) _ exprs:block_expr() _ {
            Node::FunctionDefinition(proto, exprs)
        }

        pub rule parse() -> SyntaxTree = _ funcs:(func() / extern_func())* _ {
            SyntaxTree { members: funcs }
        }
    }
}
