use jiuzhang::check::{self, Severity};
use jiuzhang::count::{self, RULES};
use jiuzhang::solve::{self, METHODS};
use jiuzhang::{board::Board, corpus, readings, report};
use std::io::Read;
use std::process::exit;

const USAGE: &str = "jiuzhang: chapter 8 of the Nine Chapters (九章算術, 方程), run on a counting board

  jiuzhang FILE              the board, worked a line at a time, with every figure the source states checked
  jiuzhang no 13             a problem of chapter 8 (also: jiuzhang no liu-hui-18-old; jiuzhang list)
  jiuzhang --plain FILE      the page, checked, without the boards
  jiuzhang --check FILE      what is wrong with it, a line each
  jiuzhang verify            run every page and compare with the source
  jiuzhang answers           how each answer is written, and which reading of the last step writes it so
  jiuzhang counts            the counts of 算 the commentators give, under every rule
  jiuzhang readings FILE     every combination of the readings a page gives in the alternative
  jiuzhang solve 3 2 1 39 / 2 3 1 34 / 1 2 3 26 [by text|reducing|cross|one]
  jiuzhang methods           the procedures `Solve` can follow
  jiuzhang rules             the rules for counting 算
  jiuzhang decisions         the choices this reconstruction makes that the text does not
  jiuzhang lsp               a language server on standard input and output
  jiuzhang json FILE         the run, as JSON, for the playground

  FILE may be - for standard input.
";

fn read(path: &str) -> String {
    if path == "-" {
        let mut s = String::new();
        std::io::stdin().read_to_string(&mut s).unwrap();
        return s;
    }
    std::fs::read_to_string(path).unwrap_or_else(|e| {
        eprintln!("jiuzhang: {}: {}", path, e);
        exit(2)
    })
}

fn page(key: &str) -> &'static str {
    match corpus::find(key) {
        Some((_, text)) => text,
        None => {
            eprintln!("jiuzhang: no page `{}` (jiuzhang list)", key);
            exit(2)
        }
    }
}

fn show(text: &str, boards: bool) -> i32 {
    let run = check::run(text);
    print!("{}", check::render(&run, boards));
    if run.findings.iter().any(|f| f.severity <= Severity::Wrong) {
        1
    } else {
        0
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let a: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
    let code = match a.as_slice() {
        [] | ["help"] | ["--help"] | ["-h"] => {
            print!("{}", USAGE);
            0
        }
        ["--version"] => {
            println!("jiuzhang {}", env!("CARGO_PKG_VERSION"));
            0
        }
        ["list"] => {
            for (id, text) in corpus::CORPUS {
                let run = check::run(text);
                println!("{:<18} {}", id, run.title().unwrap_or_default());
            }
            0
        }
        ["no", k] => show(page(k), true),
        ["no", "--plain", k] | ["--plain", "no", k] => show(page(k), false),
        ["--plain", f] => show(&read(f), false),
        ["--check", f] => {
            let run = check::run(&read(f));
            print!("{}", check::problems(&run));
            if run.findings.iter().any(|f| f.severity <= Severity::Fails) {
                1
            } else {
                0
            }
        }
        ["verify"] => report::verify(),
        ["answers"] => report::answers(),
        ["counts"] => report::counts(),
        ["readings", f] => readings::report(&read(f)),
        ["readings", "no", k] => readings::report(page(k)),
        ["methods"] => {
            for m in METHODS {
                println!("{:<9} {}", m.name(), m.describe());
            }
            0
        }
        ["rules"] => {
            for r in RULES {
                println!("{:<9} {}", r.name(), r.describe());
            }
            0
        }
        ["decisions"] => {
            print!("{}", report::DECISIONS);
            0
        }
        ["solve", rest @ ..] => solve_cmd(rest),
        ["json", f] => {
            println!("{}", jiuzhang::lsp::run_json(&read(f)));
            0
        }
        ["lsp"] => match jiuzhang::lsp::serve(&mut std::io::stdin().lock(), &mut std::io::stdout()) {
            Ok(()) => 0,
            Err(e) => {
                eprintln!("jiuzhang lsp: {}", e);
                1
            }
        },
        [f] if !f.starts_with('-') => show(&read(f), true),
        _ => {
            eprint!("{}", USAGE);
            2
        }
    };
    exit(code)
}

fn solve_cmd(rest: &[&str]) -> i32 {
    let (cols, method) = match rest.iter().position(|w| *w == "by") {
        Some(i) => (&rest[..i], rest.get(i + 1).copied().unwrap_or("text")),
        None => (rest, "text"),
    };
    let m = solve::Method::parse(method).unwrap_or_else(|| {
        eprintln!("jiuzhang: `{}` is not a method (jiuzhang methods)", method);
        exit(2)
    });
    let joined = cols.join(" ");
    let columns: Vec<Vec<i128>> = joined
        .split('/')
        .map(|c| c.split_whitespace().map(|w| jiuzhang::page::parse_num(w)).collect::<Option<Vec<_>>>())
        .collect::<Option<Vec<_>>>()
        .unwrap_or_else(|| {
            eprintln!("jiuzhang: columns are whole numbers, separated by /");
            exit(2)
        });
    let h = columns[0].len();
    if columns.is_empty() || columns.iter().any(|c| c.len() != h) || h < 2 {
        eprintln!("jiuzhang: every column needs the same number of figures, the 實 last");
        exit(2)
    }
    let mut text = String::from("Things");
    for i in 1..h {
        text.push_str(&format!(" #{}", i));
    }
    text.push_str(" 實\n");
    for (i, c) in columns.iter().enumerate() {
        let name = jiuzhang::board::col_name(i, columns.len());
        text.push_str(&format!("Set {} {}\n", name, c.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" ")));
    }
    text.push_str(&format!("Solve by {}.\n", m.name()));
    let run = check::run(&text);
    print!("{}", check::render(&run, true));
    if let Some(s) = run.solution() {
        let ops = s.ops();
        println!(
            "\n{}",
            RULES.iter().map(|r| format!("{} {}", r.name(), count::count(*r, &ops))).collect::<Vec<_>>().join(", ")
        );
        let _ = Board::rods;
    }
    if run.stopped.is_some() {
        1
    } else {
        0
    }
}
