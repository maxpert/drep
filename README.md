![Rust](https://github.com/maxpert/drep/workflows/Rust/badge.svg)

# drep is dynamic regular expression print

drep is a hot-reloadable regex filter for your piped input. This allows you filter stream of logs lines while changing regex filters on the fly. Here is an example usage:

```bash
tail -f /var/log/nginx/error | drep -f /etc/drep/filters.regex
```

Here `filters.regex` is an expressions file, and each line is treated as regular expressions, any changes to the `filters.regex` file will automatically update the regex filters, an example file can look like this:

```
(?i)warn(?-i)
178.\d+.\d+.\d+
```

For regular expression syntax please consult [this document](https://docs.rs/regex/1.3.9/regex/). 

## Why?

While `grep --line-buffered` can do something similar changing regex on the fly is not possible. Change filter regex on the fly is extremely useful in server/process environments where it's not possible to restart the process just to change the `grep` filter. Building on unix philosophy drep does only one job well, given bunch of regexes from an input file it can filter piped input stream of text lines.

## Features

 - Lightweight (2 threads in total).
 - Watch and reload filters file.
 - No GC overhead (written in rust).

## Building

Just clone the repo and run `cargo build --release`.
