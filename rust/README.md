```
              _                 _            __    _____          _
     /\      | |               | |          / _|  / ____|        | |
    /  \   __| |_   _____ _ __ | |_    ___ | |_  | |     ___   __| | ___
   / /\ \ / _` \ \ / / _ \ '_ \| __|  / _ \|  _| | |    / _ \ / _` |/ _ \
  / ____ \ (_| |\ V /  __/ | | | |_  | (_) | |   | |___| (_) | (_| |  __/
 /_/    \_\__,_| \_/ \___|_| |_|\__|  \___/|_|    \_____\___/ \__,_|\___|
```

## Running

```sh
$ cargo run 2016 01
```

## Profiling

To profile a solution, first install `flamegraph` (via `cargo install`).

Then, to get a flamegraph of any day:

```sh
# Cargo flamegraph uses dtrace on MacOS, which requires root.
# Passing --root rather than using sudo ensures only the compiled binary is run as root.
$ cargo flamegraph --root -- 2016 01
```

To get a flamechart (i.e. sorted by time) pass `--flamechart`.
