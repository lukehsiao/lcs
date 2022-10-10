# Longest Common Subsequence Edit Distance

Simple CLI to calculate the longest common subsequence edit distance between two
strings. This can be handy when doing something like grading exams, where
students are expected to order a certain sequence of events.

## Usage

```
Tool for calculating the longest common subsequence edit distance between two strings.


Usage: lcs <STR1> <STR2>

Arguments:
  <STR1>
  <STR2>

Options:
  -h, --help     Print help information
  -V, --version  Print version information
```

## Example

Using the example from [Wikipedia](https://en.wikipedia.org/wiki/Edit_distance):

    $ lcs kitten sitting
    dist: 5
    len(lcs): 4
    lcs: ittn

## Installation

First, [install rust](https://rustup.rs/), then `git clone` the repository and run:

    $ cargo install --path path/to/repo --locked
