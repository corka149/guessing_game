extern crate clap;
extern crate indicatif;
extern crate reqwest;
extern crate console;

use clap::{App, Arg};
use indicatif::{HumanBytes, ProgressBar, ProgressStyle};
use reqwest::{Client, Url, UrlError};
use console::style;
use reqwest::header::{ContentLength, ContentType};
use std::fmt::Display;
use std::fs::OpenOptions;
use std::error::Error;
use std::io::{Read, Write};

fn main() {
    let matches = App::new("Rget")
        .version("0.1.0")
        .author("corka149 <corka149@mailbox.org>")
        .about("wget clone written in Rust")
        .arg(
            Arg::with_name("URL")
                .required(true)
                .takes_value(true)
                .index(1)
                .help("url to download"),
        )
        .get_matches();
    let url = matches.value_of("URL").unwrap();
    println!("{}", url);

    if let Err(why) = download(url, false) {
        eprintln!("Error: {}", why);
    };
}

fn download(target: &str, quiet_mode: bool) -> Result<(), Box<::std::error::Error>> {

    // parse url
    let url = parse_url(target)?;
    let client = Client::new();
    let mut resp = client.get(url).send()?;
    print(
        &format!(
            "HTTP request sent... {}",
            style(format!("{}", resp.status())).green()
        ),
        quiet_mode,
        false,
    );
    if resp.status().is_success() {

        let headers = resp.headers().clone();
        let ct_len = headers.get::<ContentLength>().map(|ct_len| **ct_len);

        let ct_type = headers.get::<ContentType>().unwrap();

        match ct_len {
            Some(len) => {
                print(
                    &format!(
                        "Length: {} ({})",
                        style(len).green(),
                        style(format!("{}", HumanBytes(len))).red()
                    ),
                    quiet_mode,
                    false,
                );
            }
            None => {
                print(
                    &format!("Length: {}", style("unknown").red()),
                    quiet_mode,
                    true,
                );
            }
        }

        print(
            &format!("Type: {}", style(ct_type).green()),
            quiet_mode,
            false,
        );

        let fname = target.split("/").last().unwrap();

        print(
            &format!("Saving to: {}", style(fname).green()),
            quiet_mode,
            false,
        );

        let chunk_size = match ct_len {
            Some(x) => x as usize / 99,
            None => 1024usize, // default chunk size
        };

        let mut buf: Vec<u8> = Vec::new();

        let bar = create_progress_bar(quiet_mode, fname, ct_len);

        loop {
            let mut buffer = vec![0; chunk_size];
            let bcount = resp.read(&mut buffer[..]).unwrap();
            buffer.truncate(bcount);
            if !buffer.is_empty() {
                buf.extend(buffer.into_boxed_slice().into_vec().iter().cloned());
                bar.inc(bcount as u64);
            } else {
                break;
            }
        }

        bar.finish();

        save_to_file(&mut buf, fname)?;
    }

    Ok(())

}

pub fn parse_url(url: &str) -> Result<Url, UrlError> {
    match Url::parse(url) {
        Ok(url) => Ok(url),
        Err(error) if error == UrlError::RelativeUrlWithoutBase => {
            let url_with_base = format!("{}{}", "http://", url);
            Url::parse(url_with_base.as_str())
        }
        Err(error) => Err(error),
    }
}

pub fn print<T: Display>(var: &T, quiet_mode: bool, is_error: bool) {
    // print if not in quiet mode
    if !quiet_mode {
        if is_error {
            eprintln!("{}", var);
        } else {
            println!("{}", var);
        }
    }
}

pub fn save_to_file(buf: &[u8], name: &str) -> Result<(), Box<Error>> {
    let mut f = OpenOptions::new().create(true).write(true).open(name)?;
    f.write_all(buf)?;

    Ok(())
}

fn create_progress_bar(quit_mode: bool, msg: &str, length: Option<u64>) -> ProgressBar {
    let bar = match quit_mode {
        true => ProgressBar::hidden(),
        false => {
            match length {
                Some(len) => ProgressBar::new(len),
                None => ProgressBar::new_spinner(),
            }
        }
    };

    bar.set_message(msg);
    match length.is_some() {
        true => bar
            .set_style(ProgressStyle::default_bar()
                .template("{msg} {spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} eta: {eta}")
                .progress_chars("=> ")),
        false => bar.set_style(ProgressStyle::default_spinner()),
    };

    bar
}
