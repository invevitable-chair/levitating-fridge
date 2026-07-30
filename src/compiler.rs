use crate::ast::{Program, Statement, Expr};
use std::fmt::Write;

pub fn compile_to_c(program: &Program) -> String {
    let mut out = String::new();

    writeln!(
        &mut out,
        "#include <stdio.h>\n\nint main() {{"
    ).unwrap();

    // simple variable store: just declare all as int
    let mut vars = Vec::<String>::new();

    for stmt in &program.statements {
        match stmt {
            Statement::Assignment { name, expr } => {
                if !vars.contains(name) {
                    vars.push(name.clone());
                    writeln!(&mut out, "    int {} = {};", name, emit_expr(expr)).unwrap();
                } else {
                    writeln!(&mut out, "    {} = {};", name, emit_expr(expr)).unwrap();
                }
            }
            Statement::Print { expr } => {
                writeln!(
                    &mut out,
                    "    printf(\"%d\\n\", {});",
                    emit_expr(expr)
                ).unwrap();
            }
        }
    }

    writeln!(&mut out, "    return 0;\n}}").unwrap();
    out
}

fn emit_expr(expr: &Expr) -> String {
    match expr {
        Expr::Number(n) => format!("{}", n),
        Expr::Var(v) => v.clone(),
        Expr::Add(a, b) => format!("({} + {})", emit_expr(a), emit_expr(b)),
        Expr::Sub(a, b) => format!("({} - {})", emit_expr(a), emit_expr(b)),
    }
}
