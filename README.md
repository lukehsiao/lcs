# Longest Common Subsequence Edit Distance

Simple CLI to calculate the longest common subsequence edit distance between two
strings. This can be handy when doing something like grading exams, where
students are expected to order a certain sequence of events.

## Usage

```
Compute the length of the longest common subsequences between two strings.

USAGE:
    lcs <str1> <str2>

ARGS:
    <str1>
    <str2>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information
```

## Example

Using the example from [Wikipedia](https://en.wikipedia.org/wiki/Edit_distance):

    $ lcs kitten sitting
    dist: 5
    len(lcs): 4
    lcs: ittn

## Installation

First, [install rust](https://rustup.rs/), then `git clone` the repository and run:

    $ cargo install --path path/to/repo
