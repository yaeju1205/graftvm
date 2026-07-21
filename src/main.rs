fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        repl();
        return;
    }

    let source = if args[1] == "-e" {
        // inline expression
        args[2..].join(" ")
    } else {
        // file path
        match std::fs::read_to_string(&args[1]) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("error: could not read '{}': {}", args[1], e);
                std::process::exit(1);
            }
        }
    };

    match run(&source) {
        Ok(bytecode) => {
            println!(";; bytecode generated: {} instructions", bytecode.len());
            for (i, op) in bytecode.iter().enumerate() {
                println!("{:>4}: {:?}", i, op);
            }
        }
        Err(e) => {
            eprintln!("error: {}", e);
            std::process::exit(1);
        }
    }
}

fn run(source: &str) -> Result<Vec<graftvm_bytecode::Opcode>, String> {
    let tokens = graftvm_language::lexer::lex(source);
    let parsed = graftvm_language::parser::parse(&tokens)?;
    let bytecode = graftvm_language::lower::lower(parsed)?;
    Ok(bytecode)
}

fn repl() {
    println!("Parlance v0.1 — GraftVM frontend language");
    println!("Type an expression, or :quit to exit.");
    use std::io::Write;

    loop {
        print!("> ");
        std::io::stdout().flush().unwrap();
        let mut line = String::new();
        if std::io::stdin().read_line(&mut line).is_err() || line.trim() == ":quit" {
            break;
        }
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        match run(line) {
            Ok(bc) => {
                println!(";; {} ops", bc.len());
                for op in &bc {
                    println!("   {:?}", op);
                }
            }
            Err(e) => println!(";; error: {}", e),
        }
    }
}
