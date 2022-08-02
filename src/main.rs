use std::cmp;

use clap::{crate_authors, crate_version, App, AppSettings, Arg};
use yansi::Paint;

#[derive(Debug)]
struct Args {
    str1: String,
    str2: String,
}

fn parse_args() -> Args {
    let matches = App::new("Longest Common Subsequence")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Compute the length of the longest common subsequences between two strings.")
        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::ColoredHelp)
        .arg(Arg::with_name("str1").required(true))
        .arg(Arg::with_name("str2").required(true))
        .get_matches();

    Args {
        str1: String::from(matches.value_of("str1").unwrap()),
        str2: String::from(matches.value_of("str2").unwrap()),
    }
}

/// Compute the longest common subsequence between two strings.
///
/// Adapted from https://rosettacode.org/wiki/Longest_common_subsequence#Rust
///
/// Returns (lcs_distance, lcs_len, lcs)
fn lcs(string1: &str, string2: &str) -> (usize, usize, String) {
    let total_rows = string1.len() + 1;
    let total_columns = string2.len() + 1;
    // rust doesn't allow accessing string by index
    let string1_chars = string1.as_bytes();
    let string2_chars = string2.as_bytes();

    let mut table = vec![vec![0; total_columns]; total_rows];

    for row in 1..total_rows {
        for col in 1..total_columns {
            if string1_chars[row - 1] == string2_chars[col - 1] {
                table[row][col] = table[row - 1][col - 1] + 1;
            } else {
                table[row][col] = cmp::max(table[row][col - 1], table[row - 1][col]);
            }
        }
    }

    let mut common_seq = Vec::new();
    let mut x = total_rows - 1;
    let mut y = total_columns - 1;

    while x != 0 && y != 0 {
        // Check element above is equal
        if table[x][y] == table[x - 1][y] {
            x -= 1;
        }
        // check element to the left is equal
        else if table[x][y] == table[x][y - 1] {
            y -= 1;
        } else {
            // check the two element at the respective x,y position is same
            assert_eq!(string1_chars[x - 1], string2_chars[y - 1]);
            let char = string1_chars[x - 1];
            common_seq.push(char);
            x -= 1;
            y -= 1;
        }
    }
    let len = table[total_rows - 1][total_columns - 1];
    common_seq.reverse();
    (
        (string1.len() - len) + (string2.len() - len),
        len,
        String::from_utf8(common_seq).unwrap(),
    )
}

fn main() {
    let args = parse_args();
    let (dist, len, lcs) = lcs(args.str1.as_str(), args.str2.as_str());
    println!(
        "dist: {}\nlen(lcs): {}\nlcs: {}",
        Paint::green(dist),
        len,
        lcs
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_simple() {
        let (dist, len, lcs) = lcs("asdf", "asd");
        assert_eq!(dist, 1);
        assert_eq!(len, 3);
        assert_eq!(lcs, "asd");
    }

    #[test]
    fn test_wikipedia() {
        let (dist, len, lcs) = lcs("kitten", "sitting");
        assert_eq!(dist, 5);
        assert_eq!(len, 4);
        assert_eq!(lcs, "ittn");
    }
}
