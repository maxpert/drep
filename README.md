![Rust](https://github.com/maxpert/drep/workflows/Rust/badge.svg)

# drep is dynamic regular expression print

`drep` is `grep` with dynamic reloadable filter expressions. This allows filtering stream of 
logs/lines, while changing filters on the fly.

Filter is either a regex or plain text match, provided via input file.  
Here is an example usage:

```bash
tail -f /var/log/nginx/error.log | drep -f /etc/drep/filters
```

## Demo

[![asciicast](https://asciinema.org/a/W0B5ZVOD96YEDbhb7vnKAy1HW.svg)](https://asciinema.org/a/W0B5ZVOD96YEDbhb7vnKAy1HW)

## Filter file syntax

Each line of the filters file is an expression that starts with `~`, `=`, `!=`, or `!~`. The matches will be done 
in the order filters written in the file, and if a filter matches subsequent filters won't be executed. 

 - Any line that starts with `!~` implies does not match regex, e.g: `!~"time": \d+.\d{0,2}`
 - Any line that starts with `~` implies match regex, e.g: `~"time": \d+.\d{3,}`
 - Any line that starts with `!=` implies does not contain text, e.g: `!=INFO`
 - Any line that starts with `=` implies contain text, e.g: `="total-duration"`

Everything else is ignored, as you can see from plain text.
For regular expression documentation please refer to [this document](https://docs.rs/regex/1.3.9/regex/). 

## Why?

While `grep --line-buffered` can do something similar changing regex on the fly is not possible. 
Change filter regex on the fly is extremely useful in server/process environments where it's not possible to restart 
the process just to change the `grep` filter. 

Building on unix philosophy `drep` does only one job well, given bunch of filter from an input file 
it can filter input lines to stdout.

## Features

 - Lightweight on CPU, and memory (~3MB memory foot print, and 2 threads in total).
 - Watch and reload filters file.
 - No GC pauses and memory safe (Written in Rust).
 - Plain text & regex matching (with negation support).
 
## Usage example

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

We can launch and pipe it's output `python fizzbuzz.py | drep -f filters`. Now if the contents of `filters` are:

```
~\sfizz\n
``` 

drep will only emit logs with fizz. e.g.

```
642. fizz
648. fizz
651. fizz
654. fizz
...
```

While keeping the process running without exiting you can just modify `filters` to:

```
~\sbuzz\n
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
