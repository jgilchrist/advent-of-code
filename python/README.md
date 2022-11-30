# Solution structure

Each challenge has the following files:

* `<challenge number>.in`

```
# Input for Parts 1 and 2 of each challenge
```

* `<challenge number>.py`

```python
def part1(input):
    pass

def part2(input):
    pass

# (Optional)
# This function, if defined, transforms the content of the .input file
# before it is passed to the part1 and part2 functions.
def transform_input(input):
    return input
```

# Running

To run all challenges for the default year:

```sh
$ ./run.py
```

To run all challenges for a specific year:

```sh
$ ./run.py <year>
```

To run a single challenge:

```sh
$ ./run <year> <challenge number>
```
