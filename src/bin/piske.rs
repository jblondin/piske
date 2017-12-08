extern crate wee_rl as rustyline;
extern crate piske;
extern crate sindra;

use std::io::{Read, Write};
use std::fs::File;

use rustyline::{CompletionType, Editor};
use rustyline::completion::FilenameCompleter;
use rustyline::error::ReadlineError;

use sindra::scope::Scoped;

use piske::parse;
use piske::glue;
use piske::visitor::State;

mod result { pub type Result<T> = ::std::result::Result<T, String>; }
type Result = result::Result<()>;

struct Repl<O, E> {
    editor: Editor<FilenameCompleter>,
    cout: O,
    cerr: E,

    state: piske::visitor::state::State,
    // ast: Node<Program>,
}

const HISTORY_FILE: &str = "repl_history.txt";
const PROMPT: &str = ">> ";
const STDOUT_ERRSTR: &str = "unable to write to stdout";
const STDERR_ERRSTR: &str = "unable to write to stderr";

impl<O: Write, E: Write> Repl<O, E> {
    fn new(mut cout: O, cerr: E) -> Repl<O, E> {
        let ast = parse::program("").unwrap();
        let mut state = State::default();
        glue::interpret_pipeline(&ast, &mut state).unwrap();
        state.scope = ast.item.0.annotation.borrow().scope().unwrap();

        Repl {
            editor: {
                let config = rustyline::Config::builder()
                    .history_ignore_space(true)
                    .completion_type(CompletionType::List)
                    .build();
                let completer = FilenameCompleter::new();
                let mut editor = Editor::with_config(config);
                editor.set_completer(Some(completer));
                if editor.load_history(HISTORY_FILE).is_err() {
                    writeln!(cout, "No previous history.").expect(STDOUT_ERRSTR);
                }
                editor
            },

            cout: cout,
            cerr: cerr,

            // ast: ast,
            state: state,
        }
    }


    fn start(&mut self) -> Result {
        let mut running = true;
        while running {
            let line = self.editor.readline(PROMPT);
            match line {
                Ok(line) => {
                    self.editor.add_history_entry(line.as_ref());
                    self.read_eval_print(&line)?;
                },
                Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                    running = false;
                },
                Err(err) => {
                    writeln!(self.cerr, "Read error: {}", err)
                        .map_err(|e| format!("{}: {}", STDERR_ERRSTR, e))?;
                }
            }
        }
        self.editor.save_history(HISTORY_FILE)
            .map_err(|e| format!("Unable to save history: {}", e))?;
        Ok(())
    }

    fn read_eval_print(&mut self, input: &str) -> Result {

        match glue::interpret_statement(input, &mut self.state) {
            Ok(val) => {
                writeln!(self.cout, "{}", val).unwrap();
            },
            Err(e) => {
                writeln!(self.cerr, "interpreting statement failed: {}", e).unwrap();
            }
        }

        Ok(())
    }

}

fn interpret_file(file_name: &str) {
    match File::open(file_name) {
        Ok(mut file) => {
            let mut source = String::new();
            match file.read_to_string(&mut source) {
                Ok(_) => {
                    match piske::glue::interpret(&source) {
                        Ok(_) => {},
                        Err(e) => {
                            writeln!(::std::io::stderr(), "interpreting failed: {}", e).unwrap();
                            ::std::process::exit(1);
                        }
                    }
                },
                Err(e) => {
                    writeln!(::std::io::stderr(), "file error: {}", e).unwrap();
                    ::std::process::exit(1);
                }
            }
        },
        Err(e) => {
            writeln!(::std::io::stderr(), "file error: {}", e).unwrap();
            ::std::process::exit(1);
        }
    }
}

fn main() {
    let args: Vec<String> = ::std::env::args().collect();
    if args.len() == 1 {
        // no file passed in, open REPL
        let result = Repl::new(::std::io::stdout(), ::std::io::stderr()).start();
        match result {
            Ok(_) => { ::std::process::exit(0); },
            Err(e) => {
                writeln!(::std::io::stderr(), "Error: {}", e).unwrap();
                ::std::process::exit(1);
            }
        }
    } else if args.len() == 2 {
        interpret_file(&args[1]);
    } else {
        writeln!(::std::io::stderr(), "Usage: {} [<file>]", args[0]).unwrap();
        ::std::process::exit(1);
    }
}
