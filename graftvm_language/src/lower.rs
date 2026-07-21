use std::collections::HashMap;

use graftvm_bytecode::{Opcode, Width};
use graftvm_ir::{Func, IrBuilder, Var};
use crate::parser::{Expr, ParseResult};

/// Compile parsed Parlance source into GraftVM bytecode.
pub fn lower(result: ParseResult) -> Result<Vec<Opcode>, String> {
    let mut ctx = Context::new(
        result.infixes.iter().map(|i| (i.symbol.clone(), i.precedence)).collect(),
    );
    let mut ir = IrBuilder::new();

    // Compile function definitions
    for func_def in &result.funcs {
        let params: Vec<(&str, Width)> = func_def
            .params
            .iter()
            .map(|p| (p.as_str(), Width::I64))
            .collect();
        let f = ir.func(&func_def.name, &params, Some(Width::I64));
        let ret_var = f.ret.clone().unwrap();

        // Register params and ret in scope
        for p in &f.params {
            ctx.scope.insert(p.name.clone(), p.clone());
        }
        ctx.scope.insert(format!("{}.ret", func_def.name), ret_var.clone());
        ctx.current_func.push(f);

        // Compile body into the return slot
        let body_var = lower_expr(&mut ir, &mut ctx, &func_def.body)?;
        ir.copy_var(&ret_var, &body_var);

        ctx.current_func.pop();
        ir.exit();
    }

    // Compile top-level expressions
    for expr in &result.exprs {
        lower_expr(&mut ir, &mut ctx, expr)?;
    }

    Ok(ir.build())
}

struct Context {
    scope: HashMap<String, Var>,
    infixes: HashMap<String, u32>,
    current_func: Vec<Func>,
}

impl Context {
    fn new(infixes: HashMap<String, u32>) -> Self {
        Self {
            scope: HashMap::new(),
            infixes,
            current_func: Vec::new(),
        }
    }

    fn lookup(&self, name: &str) -> Option<&Var> {
        self.scope.get(name)
    }
}

fn lower_expr(ir: &mut IrBuilder, ctx: &mut Context, expr: &Expr) -> Result<Var, String> {
    match expr {
        Expr::Int(n) => Ok(ir.i64(&format!("lit_{}", n), *n)),
        Expr::Float(n) => Ok(ir.f64(&format!("lit_{}", n), *n)),
        Expr::Bool(b) => Ok(ir.boolean(&format!("lit_{}", b), *b)),
        Expr::String(s) => Ok(ir.i64(&format!("str_{}", s.len()), s.len() as i64)),
        Expr::Var(name) => ctx
            .lookup(name)
            .cloned()
            .ok_or_else(|| format!("undefined variable: {}", name)),

        Expr::Apply { func, args } => {
            if let Expr::Var(op) = func.as_ref() {
                match op.as_str() {
                    "+" | "-" | "*" | "/" | "%" => return lower_binop(ir, ctx, op, args),
                    "<" | "<=" | ">" | ">=" => return lower_compare(ir, ctx, op, args),
                    "==" => return lower_eq(ir, ctx, args, true),
                    "!=" => return lower_eq(ir, ctx, args, false),
                    _ => {}
                }
            }

            // Generic application — treat as inline for now
            let _func_var = lower_expr(ir, ctx, func)?;
            let mut _arg_vars = Vec::new();
            for arg in args {
                _arg_vars.push(lower_expr(ir, ctx, arg)?);
            }
            // Return a dummy var (real function call mechanism TBD)
            Ok(ir.var("apply_result", Width::I64))
        }

        Expr::Lambda { params, body } => {
            ir.enter();
            for param in params.iter() {
                let v = ir.var(param.as_str(), Width::I64);
                ctx.scope.insert(param.clone(), v);
            }
            let result = lower_expr(ir, ctx, body)?;
            let outer = ir.var("lam_result", Width::I64);
            ir.copy_var(&outer, &result);
            ir.exit();
            Ok(outer)
        }

        Expr::Let { bindings, body } => {
            for (name, val_expr) in bindings {
                let val = lower_expr(ir, ctx, val_expr)?;
                let v = ir.var(name, Width::I64);
                ir.copy_var(&v, &val);
                ctx.scope.insert(name.to_string(), v);
            }
            lower_expr(ir, ctx, body)
        }

        Expr::If { cond, then, else_ } => {
            let cond_var = lower_expr(ir, ctx, cond)?;
            let zero = ir.i64("zero", 0);
            ir.neq(&cond_var, &zero); // flag = (cond != 0)

            // flag=true → cond is truthy → use `then`
            let result = ir.var("if_result", Width::I64);
            let then_var = lower_expr(ir, ctx, then)?;
            let else_var = lower_expr(ir, ctx, else_)?;

            ir.neq(&cond_var, &zero);
            ir.branch("if_then");
            ir.copy_var(&result, &else_var);
            ir.jump("if_done");
            ir.label("if_then");
            ir.copy_var(&result, &then_var);
            ir.label("if_done");
            Ok(result)
        }
    }
}

fn lower_binop(ir: &mut IrBuilder, ctx: &mut Context, op: &str, args: &[Expr]) -> Result<Var, String> {
    if args.len() != 2 {
        return Err(format!("{} expects 2 args, got {}", op, args.len()));
    }
    let lhs = lower_expr(ir, ctx, &args[0])?;
    let rhs = lower_expr(ir, ctx, &args[1])?;
    let result = ir.var(&format!("{}_res", op), Width::I64);
    match op {
        "+" => ir.add(&result, &lhs, &rhs),
        "-" => ir.sub(&result, &lhs, &rhs),
        "*" => ir.mul(&result, &lhs, &rhs),
        "/" => ir.div(&result, &lhs, &rhs),
        "%" => ir.rem(&result, &lhs, &rhs),
        _ => unreachable!(),
    }
    Ok(result)
}

fn lower_compare(ir: &mut IrBuilder, ctx: &mut Context, op: &str, args: &[Expr]) -> Result<Var, String> {
    if args.len() != 2 {
        return Err(format!("{} expects 2 args, got {}", op, args.len()));
    }
    let lhs = lower_expr(ir, ctx, &args[0])?;
    let rhs = lower_expr(ir, ctx, &args[1])?;
    let result = ir.var(&format!("{}_res", op), Width::I8);
    match op {
        "<" => ir.lt(&result, &lhs, &rhs),
        "<=" => ir.le(&result, &lhs, &rhs),
        ">" => ir.gt(&result, &lhs, &rhs),
        ">=" => ir.ge(&result, &lhs, &rhs),
        _ => unreachable!(),
    }
    Ok(result)
}

fn lower_eq(ir: &mut IrBuilder, ctx: &mut Context, args: &[Expr], eq: bool) -> Result<Var, String> {
    if args.len() != 2 {
        return Err(format!("{} expects 2 args", if eq { "==" } else { "!=" }));
    }
    let lhs = lower_expr(ir, ctx, &args[0])?;
    let rhs = lower_expr(ir, ctx, &args[1])?;
    if eq {
        ir.eq(&lhs, &rhs);
    } else {
        ir.neq(&lhs, &rhs);
    }
    let tmp = ir.var("cmp_res", Width::I8);
    let one = ir.i64("one", 1);
    let zero = ir.i64("zero", 0);
    ir.copy_var(&tmp, &one);
    ir.branch("cmp_done");
    ir.copy_var(&tmp, &zero);
    ir.label("cmp_done");
    Ok(tmp)
}
