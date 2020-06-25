use std::fs;
use std::io::BufReader;
use std::io::prelude::*;

use regex::Regex;

use crate::errors::FiltersLoadError;

#[derive(Debug, Clone)]
enum FilterType {
    Expr(Regex),
    Text(String),
    Noop,
}

#[derive(Debug, Clone)]
pub struct Filter {
    filter_type: FilterType,
    inverted: bool,
}

impl Filter {
    pub fn is_match(&self, text: &str) -> bool {
        return match &self.filter_type {
            FilterType::Expr(exp) => exp.is_match(text) ^ self.inverted,
            FilterType::Text(need) => text.contains(need) ^ self.inverted,
            FilterType::Noop => false
        };
    }
}

fn parse_filter_line(text: &str) -> Result<Filter, FiltersLoadError> {
    let inverted = text.starts_with("!");
    let exp_text = if inverted {
        &text[1..]
    } else {
        &text[0..]
    };

    let filter_type = if exp_text.starts_with("~") {
        FilterType::Expr(Regex::new(&exp_text[1..])?)
    } else if exp_text.starts_with("=") {
        FilterType::Text(String::from(&exp_text[1..]))
    } else {
        FilterType::Noop
    };

    return Ok(Filter {
        inverted,
        filter_type,
    });
}

pub fn load_filters(path: &str) -> Result<Vec<Filter>, FiltersLoadError> {
    let file = fs::File::open(path)?;
    let mut rules: Vec<Filter> = Vec::new();
    for line in BufReader::new(file).lines() {
        let line_str = line?;
        if line_str.len() == 0 {
            continue;
        }

        let filter = parse_filter_line(line_str.as_str())?;
        match &filter.filter_type {
            FilterType::Noop => {}
            _ => rules.push(filter),
        }
    }

    return Ok(rules);
}