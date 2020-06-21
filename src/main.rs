use std::fs;
use std::io;
use std::io::{BufReader, Write};
use std::io::prelude::*;
use std::result::Result;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

use notify::{RecursiveMode, Watcher, watcher};
use regex::Regex;
use structopt::StructOpt;

use cli::CliOpts;
use errors::FiltersLoadError;

mod errors;
mod cli;

fn load_filters(path: &str) -> Result<Vec<Regex>, FiltersLoadError> {
    let file = fs::File::open(path)?;
    let mut rules: Vec<Regex> = Vec::new();
    for line in BufReader::new(file).lines() {
        let line_str = line?;
        if line_str.len() == 0 {
            continue;
        }

        let exp = Regex::new(line_str.as_str())?;
        rules.push(exp);
    }

    return Ok(rules);
}

fn watch_and_reload_filters(filters: Arc<Mutex<Vec<Regex>>>, path: &str) {
    let mut stderr = io::stderr();
    let (tx, rx) = channel();
    let mut watcher = watcher(tx, Duration::from_secs(5)).unwrap();

    watcher.watch(&path, RecursiveMode::Recursive).unwrap();

    loop {
        let mut expressions: Vec<Regex> = Vec::new();
        match rx.recv() {
            Ok(_) => expressions = load_filters(path).unwrap_or(expressions),
            Err(e) => writeln!(stderr, "Error: {}", e).unwrap()
        }

        let mut curr_filters = filters.lock().unwrap();
        *curr_filters = expressions
    }
}

fn watch_config(filters: &Arc<Mutex<Vec<Regex>>>, path: &str) {
    let watch_filters = filters.clone();
    let watch_path = path.to_owned();
    thread::spawn(move || {
        watch_and_reload_filters(watch_filters, watch_path.as_str());
    });
}

fn process_line(writer: &mut dyn io::Write, input: &mut String, filters: &Mutex<Vec<Regex>>) {
    let filter_items = filters.lock().unwrap();
    for exp in filter_items.iter() {
        if exp.is_match(input.as_str()) {
            writer.write(input.as_bytes()).unwrap();
            return;
        }
    }
}

fn main() {
    let opts: CliOpts = CliOpts::from_args();
    let file_path = opts.filters_path.to_str().unwrap();
    let loaded_filters = load_filters(file_path).unwrap();
    let filters: Arc<Mutex<Vec<Regex>>> = Arc::new(Mutex::new(loaded_filters));

    let mut stdout = io::stdout();
    let mut stderr = io::stderr();

    watch_config(&filters, file_path);

    loop {
        let mut input_line = String::new();
        match io::stdin().read_line(&mut input_line) {
            Ok(_) => process_line(&mut stdout, &mut input_line, &filters),
            Err(e) => {
                writeln!(stderr, "Error {}: {}", e, input_line).unwrap();
            }
        }
    }
}
