use std::{
    error, fs,
    io::{stdin, stdout, Read, Write},
    path::PathBuf,
    process,
};

use atty::{is, Stream};
use clap::Clap;
use scraper::{Html, Selector};

type Error = Box<dyn error::Error>;

#[derive(Clap, Debug)]
#[clap(about, version)]
pub struct App {
    /// CSS Selectors to query <required>
    #[clap(
        parse(from_str = get_selector),
        validator = |input| -> Result<(), Error> {
            Selector::parse(input).map_err(|_| String::from("Invalid selector"))?;
            Ok(())
        })
    ]
    selector: Selector,

    /// Path to html file <optional>
    #[clap(validator = |input| -> Result<(), Error> {
            PathBuf::from(input).canonicalize()?;
            Ok(())
        })
    ]
    path: Option<PathBuf>,
}

fn get_selector(input: &str) -> Selector {
    // can unwrap because `selector` field is validated
    Selector::parse(input).unwrap()
}

impl App {
    pub fn run() -> Result<(), Error> {
        let Self { path, selector } = Self::parse();

        let html = if is(Stream::Stdin) {
            match path {
                Some(path) => fs::read_to_string(&path)?,
                None => {
                    eprintln!("Cannot read html, either pipe html or pass file path.\nEg:\n  $ curl -s 'https://en.wikipedia.org/wiki/Wiki' | {0} '#siteSub'\n  $ {0} '#siteSub' /path/to/file.html", env!("CARGO_PKG_NAME"));
                    process::exit(1);
                }
            }
        } else {
            // using functional style eg: stdin().lock().lines()
            // doesn't preserve input's formatting
            let mut data = String::new();
            let stdin = stdin();
            let mut handle = stdin.lock();

            handle.read_to_string(&mut data)?;

            data
        };

        let all_text = Html::parse_document(&html).select(&selector).fold(
            String::new(),
            |mut text, element| {
                let inner_text = element.text().fold(String::new(), |mut acc, curr| {
                    acc.push_str(curr);
                    acc
                });

                text.push_str(&inner_text);

                text
            },
        );

        // writing to stdout this way instead of `println!`, to prevent `Broken pipe (os error 32)`
        // error when the out is piping in linux (not sure in other os
        stdout()
            .lock()
            .write_all(format!("{}\n", all_text.trim()).as_bytes())?;

        Ok(())
    }
}
