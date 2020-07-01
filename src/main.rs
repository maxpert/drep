use std::io;
use std::io::Write;
use std::sync::Arc;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

use notify::{DebouncedEvent, RecursiveMode, Watcher, watcher};
use parking_lot::Mutex;
use structopt::StructOpt;

use cli::CliOpts;
use filter::Filter;

mod filter;
mod errors;
mod cli;


fn is_update_event(ev: &DebouncedEvent) -> bool {
    match ev {
        DebouncedEvent::Write(_)
        | DebouncedEvent::Create(_)
        | DebouncedEvent::Remove(_)
        | DebouncedEvent::Rename(_, _)
        | DebouncedEvent::Chmod(_) => true,
        _ => false
    }
}

fn load_filters(path: &str, stderr: &mut io::Stderr) -> Vec<Filter> {
    match Filter::load(path) {
        Ok(res) => res,
        Err(e) => {
            writeln!(stderr, "Error loading filters {}, defaulting to empty filters", e).unwrap();
            Vec::new()
        }
    }
}

fn watch_and_reload_filters(filters: Arc<Mutex<Vec<Filter>>>, path: &str) {
    let mut stderr = io::stderr();
    let (tx, rx) = channel();
    let mut watcher = watcher(tx, Duration::from_millis(100)).unwrap();

    watcher.watch(&path, RecursiveMode::NonRecursive).unwrap();

    loop {
        match rx.recv() {
            Ok(ev) if is_update_event(&ev) => {
                let expressions = load_filters(path, &mut stderr);
                let mut curr_filters = filters.lock();
                *curr_filters = expressions;
            }
            Ok(_) => {},
            Err(e) => writeln!(stderr, "Error: {}", e).unwrap()
        }
    }
}

fn watch_config(filters: &Arc<Mutex<Vec<Filter>>>, path: &str) {
    let watch_filters = filters.clone();
    let watch_path = String::from(path);
    thread::spawn(move || {
        watch_and_reload_filters(watch_filters, watch_path.as_str());
    });
}

#[inline]
fn is_match(filters: &Mutex<Vec<Filter>>, input: &mut String) -> bool {
    let filter_items = filters.lock();
    for filter in filter_items.iter() {
        if filter.is_match(input.as_str()) {
            return true
        }
    }

    false
}

fn process_line(writer: &mut dyn io::Write, input: &mut String, filters: &Mutex<Vec<Filter>>) {
    if is_match(filters, input) {
        writer.write_all(input.as_bytes()).unwrap();
    }
}

fn main() {
    let opts: CliOpts = CliOpts::from_args();
    let file_path = opts.filters_path.to_str().unwrap();
    let loaded_filters = match Filter::load(file_path) {
        Ok(v) => v,
        Err(e) => {
            println!("drep failed {}", e);
            return;
        }
    };
    let filters = Arc::new(Mutex::new(loaded_filters));

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
