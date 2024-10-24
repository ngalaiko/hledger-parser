use ariadne::{Color, Config, IndexType, Label, Report, ReportKind, Source};
use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[arg(long, env)]
    ledger_file: std::path::PathBuf,
}

#[allow(clippy::missing_panics_doc)]
pub fn main() {
    let cli = Cli::parse();
    let contents = match std::fs::read_to_string(cli.ledger_file) {
        Ok(contents) => contents,
        Err(error) => {
            println!("{error}");
            std::process::exit(1);
        }
    };

    let result = hledger_parser::parse(&contents);
    match result {
        Ok(directives) => {
            println!("{directives:#?}");
            std::process::exit(1);
        }
        Err(errs) => {
            for err in errs {
                Report::build(ReportKind::Error, (), err.span().start)
                    .with_code(3)
                    .with_config(Config::default().with_index_type(IndexType::Byte))
                    .with_message(err.to_string())
                    .with_label(
                        Label::new(err.span().into_range())
                            .with_message(err.reason().to_string())
                            .with_color(Color::Red),
                    )
                    .finish()
                    .eprint(Source::from(&contents))
                    .expect("should build report");
            }
        }
    }
}
