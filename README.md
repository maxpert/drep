![Rust](https://github.com/maxpert/drep/workflows/Rust/badge.svg)

# drep is dynamic regular expression print

drep is a auto reloaded regex from file to filter your piped input lines. This allows you filter stream of logs/lines while changing regex filters on the fly. Here is an example usage:

```bash
tail -f /var/log/nginx/error | drep -f /etc/drep/filters.regex
```

Here `filters.regex` is an expressions file, and each line is treated as regular expressions, any changes to the `filters.regex` file will automatically update the regex filters, an example file can look like this:

```
(?i)warn(?-i)
178.\d+.\d+.\d+
```

For regular expression documentation please refer to [this document](https://docs.rs/regex/1.3.9/regex/). 

## Why?

While `grep --line-buffered` can do something similar changing regex on the fly is not possible. Change filter regex on the fly is extremely useful in server/process environments where it's not possible to restart the process just to change the `grep` filter. Building on unix philosophy drep does only one job well, given bunch of regexes from an input file it can filter piped input stream of text lines.

## Features

 - Lightweight on CPU, and memory (<5MB memory foot print, and 2 threads in total).
 - Watch and reload filters file.
 - No GC pauses and memory safe (Written in Rust).
 
## Usage tutorial

Given following simple `fizzbuzz.py`:

```python
import time

i = 1
while True:
    fb = ""
    if i % 3 == 0:
        fb = "fizz"
    if i % 5 == 0:
        fb = "{}buzz".format(fb)

    if fb:
        print("{}. {}".format(i, fb), flush=True)

    i = i + 1
    time.sleep(0.1)
```

We can launch and pipe it's output `python fizzbuzz.py | drep -f filters.regex`. Now if the contents of `filters.regex` are:

```
\sfizz\n
``` 

drep will only emit logs with fizz. e.g.

```
642. fizz
648. fizz
651. fizz
654. fizz
...
```

While keeping the process running without exiting you can just modify `filters.regex` to:

```
\sbuzz\n
```

This will change the drep output on the fly to only emit buzz:
```
805. buzz
815. buzz
820. buzz
...
```

## Building

Just clone the repo and run `cargo build --release`.