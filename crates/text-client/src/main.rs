extern crate term_table;
#[macro_use]
extern crate structopt;

#[macro_use] extern crate quicli;
use quicli::prelude::*;
use term_table::{Table,TableStyle};
use term_table::{row::Row,
    cell::{Alignment,Cell},
};

// Add cool slogan for your app here, e.g.:
/// Get first n lines of a file
#[derive(Debug, StructOpt)]
struct Cli {
    // Add a CLI argument `--count`/-n` that defaults to 3, and has this help text:
    /// How many lines to get
    #[structopt(long = "count", short = "n", default_value = "3")]
    count: usize,
    // Add a positional argument that the user has to supply:
    /// The sql to execute
    sql: String,
    /// the sql file to execute
    #[structopt(long = "file", short = "f")]
    file: Option<String>,
    // Quick and easy logging setup you get for free with quicli
    #[structopt(flatten)]
    verbosity: Verbosity,
}

main!(|args: Cli, log_level: verbosity| {
    println!("Executing {}", args.sql);
    if let Some(ref file) = args.file{
        let content = read_file(file)?;
        let content_lines = content.lines();
        let first_n_lines = content_lines.take(args.count);
        info!("Reading first {} lines of {:?}", args.count, args.file);
        for line in first_n_lines {
            println!("{}", line);
        }
    }
    if !args.sql.is_empty(){
        exec(&args.sql);
    }
});

fn exec(sql: &str){
    let mut table = Table::new();
    table.max_column_width = 40;

    table.style = TableStyle::elegant();

    table.add_row(Row::new(vec![
        Cell::new_with_alignment("This is some centered text", 2, Alignment::Center)
    ]));

    table.add_row(Row::new(vec![
        Cell::new("This is left aligned text", 1),
        Cell::new_with_alignment("This is right aligned text", 1, Alignment::Right)
    ]));

    table.add_row(Row::new(vec![
        Cell::new("This is left aligned text", 1),
        Cell::new_with_alignment("This is right aligned text", 1, Alignment::Right)
    ]));

    table.add_row(Row::new(vec![
        Cell::new("This is some really really really really really really really really really that is going to wrap to the next line", 2),
    ]));

    println!("{}", table.as_string());
}
