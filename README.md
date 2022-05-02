# date-diff

Command line tool to calculate the difference between two dates

## Install

```
git clone
cd date-diff
cargo install --path .
```

## Usage

```
$ date-diff 1970-1-1 2022-5-2
```

output

```
years: 52, months: 4, weeks: 2730, days: 19114, hours: 458736, minutes: 27524160, seconds: 1651449600
```

### Help

```
Calculate the difference between two dates

USAGE:
    date-diff [OPTIONS] <START_DATE> <END_DATE>

ARGS:
    <START_DATE>    Date formats: yyyy-mm-dd or yyyy/mm/dd
    <END_DATE>      Date formats: yyyy-mm-dd or yyyy/mm/dd

OPTIONS:
    -d, --days
    -h, --hours
        --help       Print help information
    -m, --months
    -M, --minutes
    -s, --seconds
    -V, --version    Print version information
    -w, --weeks

```
