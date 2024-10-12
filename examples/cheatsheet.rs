use ariadne::{Color, Label, Report, ReportKind, Source};

const CHEATSHEET_JOURNAL: &str = include_str!("./fixture/cheatsheet.journal");

pub fn main() {
    let result = hledger_parser::parse(CHEATSHEET_JOURNAL);

    match result {
        Ok(directives) => println!("{directives:#?}"),
        Err(errs) => {
            for err in errs {
                Report::build(ReportKind::Error, (), err.span().start)
                    .with_code(3)
                    .with_message(err.to_string())
                    .with_label(
                        Label::new(err.span().into_range())
                            .with_message(err.reason().to_string())
                            .with_color(Color::Red),
                    )
                    .finish()
                    .eprint(Source::from(CHEATSHEET_JOURNAL))
                    .unwrap();
            }
        }
    }
}
