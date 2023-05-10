use std::{
    io::stdout,
    path::{Path, PathBuf},
};

use ariadne::{
    sources, Cache, ColorGenerator, Config, FileCache, FnCache, Label, LabelAttach, Report,
    ReportKind, Source,
};
use chumsky::span::Span;
use crane_lex as lex;
use crane_parse as parse;

use parse::package::{pass::PrintPackage, Package};

fn main() -> anyhow::Result<()> {
    env_logger::init();
    let args = std::env::args();

    let mut cache = FileCache::default();

    let (contents, filename) = if let Some(file) = args.skip(1).next() {
        (cache.fetch(&Path::new(&file)), file)
    } else {
        println!("Usage: crc <file>");
        return Err(anyhow::anyhow!("No file specified"));
    };

    let contents_str = contents.unwrap().chars().collect::<String>();
    let lexer_res = lex::lex(&*contents_str, "test")?;

    let errors = lexer_res.errors();

    if errors.len() > 0 {
        println!("Encountered lexer errors");
    }
    for error in errors {
        // println!("{}", error);
        let mut label = Label::new((&filename, error.span().start()..error.span().end()));
        for (msg, span) in error.contexts() {
            label = label.with_message(msg);
        }
        let mut report = Report::build(ReportKind::Error, &filename, error.span().start())
            .with_message(error.reason())
            .with_label(label)
            .finish()
            .print((&filename, Source::from(&contents_str)));
        // .print(sources(vec![(filename.as_str(), contents_str.as_str())]));

        // report.eprint(cache)?;
        // let mut s = stdout();
        // report.write(cache, std::io::BufWriter::new(&mut s));
    }

    if !lexer_res.has_output() {
        return Err(anyhow::anyhow!("Lexer failed"));
    }

    let tokens = lexer_res.into_output().unwrap();

    // println!(
    //     "{:#?}",
    //     tokens
    //         .iter() /* .rev().take(5).rev()*/
    //         .collect::<Vec<_>>()
    //
    // );

    let mut package = Package::new();

    let _ = {
        let parsed = parse::parse(tokens.clone(), &mut package, "test".to_owned())?;
        let errors = parsed.errors();
        if errors.len() > 0 {
            for error in errors {
                let start = error.span().start();

                let start_token = tokens.get(start).unwrap();
                let source = cache.fetch(Path::new(&filename)).unwrap();

                let start = start_token.span.start();
                let end = start_token.span.end();

                let expected = Label::new((&filename, start..end))
                    .with_message(format!(
                        "Expected one of {}",
                        error
                            .expected()
                            .into_iter()
                            .take(8)
                            .map(|v| v.to_string())
                            .collect::<Vec<_>>()
                            .join(", "),
                    ))
                    .with_color(ariadne::Color::Green);

                let found = Label::new((&filename, start..end))
                    .with_message(format!(
                        "Unexpected token '{}'",
                        error
                            .found()
                            .map(|v| v.to_string())
                            .unwrap_or("".to_string())
                    ))
                    .with_color(ariadne::Color::Red);

                let report = Report::build(ReportKind::Error, &filename, start)
                    .with_message(error.reason())
                    .with_label(found)
                    .with_label(expected);

                report.finish().print((&filename, source.clone()))?;
            }
            return Ok(());
        }
    };

    // Inspect the unit / package AST trees
    //
    // let id = parsed.into_output().unwrap();
    // let unit = package.unit(id).unwrap();
    // println!("{:#?}", unit);
    // println!("{:#?}", package);

    // Debug print pass (reconstruct code based on AST)
    println!("Reconstructed from AST:\n");
    package.inspect(&mut PrintPackage, ());
    Ok(())
}
