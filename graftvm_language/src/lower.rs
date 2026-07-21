use std::collections::HashMap;

use graftvm_bytecode::{Opcode, Width};
use graftvm_ir::{IrBuilder, Var};
use crate::parser::{Expr, ParseResult};

/// Compile parsed Parlance source into GraftVM bytecode.
pub fn lower(result: ParseResult) -> Result<Vec<Opcode>, String> {
    let mut ctx = Context::new(
        result.infixes.iter().map(|i| (i.symbol.clone(), i.precedence)).collect(),
    );
    let mut ir = IrBuilder::new();

    // Register function names so recursive refs work.
    for func_def in &result.funcs {
        let _p = ir.var(&func_def.name, Width::I64);
        ctx.scope.insert(func_def.name.clone(), _p);
    }

    // Compile function definitions — each runs inline at definition time.
    for func_def in &result.funcs {
        ir.enter();
        for param in &func_def.params {
            let v = ir.var(param, Width::I64);
            ctx.scope.insert(param.clone(), v);
        }
        let body_var = lower_expr(&mut ir, &mut ctx, &func_def.body)?;
        let ret = ir.var(&format!("{}.ret", func_def.name), Width::I64);
        ir.copy_var(&ret, &body_var);
        ir.exit();
        for param in &func_def.params {
            ctx.scope.remove(param);
        }
    }

    // Compile top-level expressions
    for expr in &result.exprs {
        lower_expr(&mut ir, &mut ctx, expr)?;
    }

    Ok(ir.build())
}

struct Context {
    scope: HashMap<String, Var>,
    #[allow(dead_code)]
    infixes: HashMap<String, u32>,
}

impl Context {
    fn new(infixes: HashMap<String, u32>) -> Self {
        Self { scope: HashMap::new(), infixes }
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
        Expr::Var(name) => ctx.lookup(name).cloned().ok_or_else(|| format!("undefined variable: {}", name)),

        Expr::Apply { func, args } => {
            if let Expr::Var(op) = func.as_ref() {
                match op.as_str() {
                    "+" | "-" | "*" | "/" | "%" => return lower_binop(ir, ctx, op, args),
                    "<" | "<=" | ">" | ">=" => return lower_compare(ir, ctx, op, args),
                    "==" => return lower_eq(ir, ctx, args, true),
                    "!=" => return lower_eq(ir, ctx, args, false),
                    _ => {} // user function — treat as opaque
                }
            }
            // Generic: just lower args, return dummy
            let _f = lower_expr(ir, ctx, func)?;
            for arg in args { lower_expr(ir, ctx, arg)?; }
            Ok(ir.var("apply", Width::I64))
        }

        Expr::Lambda { params, body } => {
            ir.enter();
            for param in params.iter() {
                let v = ir.var(param.as_str(), Width::I64);
                ctx.scope.insert(param.clone(), v);
            }
            let result = lower_expr(ir, ctx, body)?;
            let outer = ir.var("lam_res", Width::I64);
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
            let then_var = lower_expr(ir, ctx, then)?;
            let else_var = lower_expr(ir, ctx, else_)?;
            let zero = ir.i64("zero", 0);
            let result = ir.var("if_res", Width::I64);
            ir.neq(&cond_var, &zero);
            ir.branch("if_t");
            ir.copy_var(&result, &else_var);
            ir.jump("if_d");
            ir.label("if_t");
            ir.copy_var(&result, &then_var);
            ir.label("if_d");
            Ok(result)
        }
    }
}

fn lower_binop(ir: &mut IrBuilder, ctx: &mut Context, op: &str, args: &[Expr]) -> Result<Var, String> {
    if args.len() != 2 { return Err(format!("{} expects 2 args", op)); }
    let lhs = lower_expr(ir, ctx, &args[0])?;
    let rhs = lower_expr(ir, ctx, &args[1])?;
    let r = ir.var(&format!("{}_r", op), Width::I64);
    match op {
        "+" => ir.add(&r, &lhs, &rhs),
        "-" => ir.sub(&r, &lhs, &rhs),
        "*" => ir.mul(&r, &lhs, &rhs),
        "/" => ir.div(&r, &lhs, &rhs),
        "%" => ir.rem(&r, &lhs, &rhs),
        _ => unreachable!(),
    }
    Ok(r)
}

fn lower_compare(ir: &mut IrBuilder, ctx: &mut Context, op: &str, args: &[Expr]) -> Result<Var, String> {
    if args.len() != 2 { return Err(format!("{} expects 2 args", op)); }
    let lhs = lower_expr(ir, ctx, &args[0])?;
    let rhs = lower_expr(ir, ctx, &args[1])?;
    let r = ir.var(&format!("{}_r", op), Width::I8);
    match op {
        "<" => ir.lt(&r, &lhs, &rhs),
        "<=" => ir.le(&r, &lhs, &rhs),
        ">" => ir.gt(&r, &lhs, &rhs),
        ">=" => ir.ge(&r, &lhs, &rhs),
        _ => unreachable!(),
    }
    Ok(r)
}

fn lower_eq(ir: &mut IrBuilder, ctx: &mut Context, args: &[Expr], eq: bool) -> Result<Var, String> {
    if args.len() != 2 { return Err(format!("== expects 2 args")); }
    let lhs = lower_expr(ir, ctx, &args[0])?;
    let rhs = lower_expr(ir, ctx, &args[1])?;
    if eq { ir.eq(&lhs, &rhs); } else { ir.neq(&lhs, &rhs); }
    let tmp = ir.var("cmp_r", Width::I8);
    let one = ir.i64("one", 1);
    let zero = ir.i64("zero", 0);
    ir.copy_var(&tmp, &one);
    ir.branch("cmp_d");
    ir.copy_var(&tmp, &zero);
    ir.label("cmp_d");
    Ok(tmp)
}
