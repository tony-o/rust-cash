# Cash (Money, Millionaires)

`cash` is a command line tool to cache program output so you can grep/awk/whatever without polluting local dir.

## Usage

```
$ cash find /         #this caches the output
# output
$ cash | grep 'xyz'   #this pipes output to grep
```

## Build

Requires cargo.

```
$ cargo build --release
$ sudo ln -s $(pwd)/target/release/cash /usr/local/bin/cash
```

## Args

### -n <name>

This can be used to name the output. Usage with this is similar to above,

```
$ cash -n test find /
# output
$ cash -n test | grep 'xyz'
```

### -d <dir>

If you want to use a special dir for caching, use this flag.  If you want to permanently use this dir then use an alias with your shell.

## License

[![License: Artistic-2.0](https://img.shields.io/badge/License-Artistic%202.0-0298c3.svg)](https://opensource.org/licenses/Artistic-2.0)

## authors

@[tony-o](https://github.com/tony-o)
