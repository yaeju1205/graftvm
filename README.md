# GraftVM

A register-based virtual machine with a graft-style windowed memory model,
written in Rust.

## Repository Structure

```
graftvm_liternal/    — Tagged literal types (Int/UInt/Float/Bool/String)
graftvm_bytecode/    — Opcode definitions (30 variants with Width parameterization)
graftvm_window/      — Window memory model (slots, stack)
graftvm_interpreter/ — Bytecode VM (the engine)
graftvm_ir/          — High-level IR builder (variables, labels, functions, scopes)
graftvm_language/    — Parlance frontend (lexer → parser → bytecode lowering)
src/                 — Binary: REPL and file runner
```

## Quick Start

```bash
# Run a Parlance source file
cargo run -- examples/fact.gfvm

# Run with instruction trace
cargo run -- -d examples/fact.gfvm

# Start the REPL
cargo run

# Evaluate inline
cargo run -- -e "(+ 1 2)"
```

## Parlance Language

Parlance is a minimal S-expression-based language that compiles directly
to GraftVM bytecode and runs it on the built-in interpreter.

### Syntax

```scheme
;; Literals
42
3.14
true
false
"hello"

;; Arithmetic
(+ 1 2)
(* (+ 3 4) 5)

;; Variables and let bindings
(let ((x 42)) (+ x 1))

;; Conditions
(if (< n 2) 1 (* n (fact (- n 1))))

;; Functions
(def add x y (+ x y))

;; Lambdas
(lambda (x) (* x x))

;; Infix operator declarations
(infix + 3)
(infix * 4)
```

### Examples

**Fibonacci** (`examples/fib.gfvm`):
```scheme
(infix + 3)
(infix - 3)
(infix < 2)

(def fib n
  (if (< n 2)
      n
      (+ (fib (- n 1)) (fib (- n 2)))))

(fib 10)
```

**Factorial** (`examples/fact.gfvm`):
```scheme
(infix * 4)
(infix < 2)

(def fact n
  (if (< n 2)
      1
      (* n (fact (- n 1)))))

(fact 5)
```

### Flags

| Flag | Description |
|------|-------------|
| `-d` / `--debug` | Print bytecode dump + execution trace + final VM state |
| `-e` | Evaluate an inline expression |

Example:
```bash
cargo run -- -d -e "(+ 1 2)"
```

## Architecture

```
Parlance source
     │
     ▼
  lexer.rs    → Token stream
     │
     ▼
  parser.rs   → Expr AST
     │
     ▼
  lower.rs    → graftvm_ir::IrBuilder
     │
     ▼
  Opcode      → graftvm_interpreter::VM
     │
     ▼
  dump_state  → slot values, cmp flag
```

The language frontend (`graftvm_language`) converts Parlance source into
GraftVM bytecode through a three-stage pipeline — lexing, parsing, and lowering
via the high-level IR builder (`graftvm_ir`). The resulting bytecode is then
executed by the interpreter, and the final VM state is displayed.

## License

MIT
