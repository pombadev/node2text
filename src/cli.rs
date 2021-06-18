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
        long_about = "CSS Selectors to query <required>\nTakes multiple values, eg: node2text '#head','#body','#footer'",
        validator = selectors_validator
    )]
    selectors: Vec<String>,

    /// Path to html file <optional>
    #[clap(
        last = true,
        validator = |input| -> Result<(), Error> {
            PathBuf::from(input).canonicalize()?;
            Ok(())
        })
    ]
    path: Option<PathBuf>,
}

fn selectors_validator(input: &str) -> Result<(), Error> {
    let invalids = input
        .split(",")
        .filter_map(|s| {
            if Selector::parse(s).is_err() {
                Some(s)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    if invalids.is_empty() {
        Ok(())
    } else {
        Err(format!("Failed parsing selector(s): {}", invalids.join(", ")).into())
    }
}

impl App {
    pub fn run() -> Result<(), Error> {
        let Self { path, selectors } = Self::parse();

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

        let all_text = selectors
            .iter()
            .filter_map(|input| Selector::parse(input).ok())
            .fold(String::new(), |mut all, selector| {
                let text_for_selector = Html::parse_document(&html).select(&selector).fold(
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

                all.push_str(&text_for_selector);

                all
            });

        let all_text = all_text.trim();

        if all_text.is_empty() {
            // selector was probably not found in the html, notify user
            process::exit(1);
        }

        // writing to stdout this way instead of `println!`, to prevent `Broken pipe (os error 32)`
        // error when the out is piping in linux (not sure in other os
        stdout()
            .lock()
            .write_all(format!("{}\n", all_text).as_bytes())?;

        Ok(())
    }
}
