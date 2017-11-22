extern crate rustyline;
extern crate piske;
extern crate sindra;

use std::io::Write;

use rustyline::{CompletionType, Editor};
use rustyline::completion::FilenameCompleter;
use rustyline::error::ReadlineError;

use sindra::scope::Scoped;

use piske::parse;
use piske::interpret;
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
        interpret::pipeline(&ast, &mut state).unwrap();
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

        match interpret::interpret_statement(input, &mut self.state) {
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

fn main() {
    let result = Repl::new(::std::io::stdout(), ::std::io::stderr()).start();
    match result {
        Ok(_) => { ::std::process::exit(0); },
        Err(e) => {
            writeln!(::std::io::stderr(), "Error: {}", e).unwrap();
            ::std::process::exit(1);
        }
    }
}
