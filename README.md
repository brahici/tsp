# tsp: timestamp parser

tsp is a command line tool for converting timestamps to dates.

## Install

Either use `cargo build` or provided `Makefile`.

By default, install copies the binary to `/usr/local/bin`. Target directory can be overriden with `TARGET` variable.

```console
TARGET=~/local/.bin make install
[...]
```

## Quickstart

### Convert a timestamp in seconds

```console
$ tsp 1758643530
1758643530           :: Tue, 23 Sep 2025 16:05:30 +0000
```

### Convert a timestamp in milliseconds

```console
$ tsp m1758643530
m1758643530          :: Wed, 21 Jan 1970 08:30:43 +0000
```

### Multiple values with different prefixes can be passed

```console
$ tsp 1758643530 s1758643530 m1758643530 u1758643530 n1758643530
1758643530           :: Tue, 23 Sep 2025 16:05:30 +0000
s1758643530          :: Tue, 23 Sep 2025 16:05:30 +0000
m1758643530          :: Wed, 21 Jan 1970 08:30:43 +0000
u1758643530          :: Thu, 01 Jan 1970 00:29:18 +0000
n1758643530          :: Thu, 01 Jan 1970 00:00:01 +0000
```

### Failures are reported but don't break the whole processing

```console
$ tsp 1758643530 not_a_ts m1758643530
1758643530           :: Tue, 23 Sep 2025 16:05:30 +0000
not_a_ts             :: the value is not an integer
m1758643530          :: Wed, 21 Jan 1970 08:30:43 +0000
```
