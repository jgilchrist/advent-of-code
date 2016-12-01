```
              _                 _            __    _____          _
     /\      | |               | |          / _|  / ____|        | |
    /  \   __| |_   _____ _ __ | |_    ___ | |_  | |     ___   __| | ___
   / /\ \ / _` \ \ / / _ \ '_ \| __|  / _ \|  _| | |    / _ \ / _` |/ _ \
  / ____ \ (_| |\ V /  __/ | | | |_  | (_) | |   | |___| (_) | (_| |  __/
 /_/    \_\__,_| \_/ \___|_| |_|\__|  \___/|_|    \_____\___/ \__,_|\___|
```

# Solution structure

Each challenge has the following files:

* [challenge number].input

```
# Input for Parts 1 and 2 of each challenge
```

* [challenge number].py

```python
# (Optional)
# This function, if defined, transforms the content of the .input file
# before it is passed to the part1 and part2 functions.
def transform_input(challenge_input):
	return challenge_input

def part1(input):
    pass

def part2(input):
    pass
```

# Running challenges

To run all challenges in a directory:

```sh
$ ./run
```

To run a single challenge:

```sh
$ ./run [challenge number]
```
