# Longest Common Subsequence Edit Distance

Simple CLI to calculate the longest common subsequence edit distance between two
strings. This can be handy when doing something like grading exams, where
students are expected to order a certain sequence of events.

## Usage

```
Compute the length of the longest common subsequences between two strings.

USAGE:
    lcs <str1> <str2>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <str1>
    <str2>
```

## Example

Using the example from [Wikipedia](https://en.wikipedia.org/wiki/Edit_distance):

    $ lcs kitten sitting
    dist: 5
    len(lcs): 4
    lcs: ittn

## Installation

First, [install rust](https://rustup.rs/), then close the repository and run:

    $ cargo install --path path/to/repo
