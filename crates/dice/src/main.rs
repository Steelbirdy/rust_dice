use parser::parse;
use std::io::{self, Write};


fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    let mut input = String::new();

    loop {
        write!(stdout, "> ")?;
        stdout.flush()?;

        stdin.read_line(&mut input)?;

        let parse = parse(&input);
        println!("{}", parse.debug_tree());

        let syntax = parse.syntax();

        for error in ast::validation::validate(&syntax) {
            println!("{}", error);
        }

        let root = ast::Root::cast(syntax).unwrap();

        dbg!(root.expr());

        let mut roll_result = hir::roll(root);
        let total = roll_result.total();
        dbg!(roll_result);

        println!("Total: {}", total);

        input.clear();
    }
}