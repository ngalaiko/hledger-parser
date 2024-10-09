use ariadne::{Color, Fmt, Label, Report, ReportKind, Source};

const CHEATSHEET_JOURNAL: &str = include_str!("./fixture/cheatsheet.journal");

pub fn main() {
    let result = hledger_parser::parse(CHEATSHEET_JOURNAL);

    match result {
        Ok(directives) => println!("{directives:#?}"),
        Err(errs) => {
            for err in errs {
                let msg = if let chumsky::error::SimpleReason::Custom(msg) = err.reason() {
                    msg.clone()
                } else {
                    format!(
                        "{}{}",
                        if err.found().is_some() {
                            "Unexpected token"
                        } else {
                            "Unexpected end of input"
                        },
                        if let Some(label) = err.label() {
                            format!(" while parsing {label}")
                        } else {
                            String::new()
                        },
                    )
                };

                let report = Report::build(ReportKind::Error, (), err.span().start)
                    .with_code(3)
                    .with_message(msg)
                    .with_label(
                        Label::new(err.span())
                            .with_message(
                                if let chumsky::error::SimpleReason::Custom(msg) = err.reason() {
                                    msg.clone()
                                } else {
                                    format!(
                                        "Unexpected {}",
                                        err.found().map_or_else(
                                            || "end of input".to_string(),
                                            |c| format!("token {}", c.fg(Color::Red))
                                        )
                                    )
                                },
                            )
                            .with_color(Color::Red),
                    );

                let report = match err.reason() {
                    chumsky::error::SimpleReason::Unclosed { span, delimiter } => report
                        .with_label(
                            Label::new(span.clone())
                                .with_message(format!(
                                    "Unclosed delimiter {}",
                                    delimiter.fg(Color::Yellow)
                                ))
                                .with_color(Color::Yellow),
                        ),
                    chumsky::error::SimpleReason::Unexpected
                    | chumsky::error::SimpleReason::Custom(_) => report,
                };

                report
                    .finish()
                    .print(Source::from(&CHEATSHEET_JOURNAL))
                    .unwrap();
            }
        }
    }
}
