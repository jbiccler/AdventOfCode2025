// Advent of Code parsing helpers.
// All functions take `input: &str` and return parsed structures.

use std::fmt::Debug;
use std::str::FromStr;

/// Return lines as Vec<&str> (no empty final line).
pub fn lines(input: &str) -> Vec<&str> {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.trim())
        .collect()
}

/// Parse each line with a custom parser function.
pub fn map_lines<T>(input: &str, f: impl Fn(&str) -> T) -> Vec<T> {
    input.lines().filter(|l| !l.is_empty()).map(f).collect()
}

/// Generic, works with any type that implements FromStr
pub fn parse_lines<T: FromStr>(input: &str) -> Vec<T>
where
    <T as FromStr>::Err: Debug,
{
    input
        .lines()
        .map(|line| line.trim()) // Handle potential carriage returns or spaces
        .filter(|line| !line.is_empty()) // Skip empty lines at end of file
        .map(|line| line.parse::<T>().expect("Failed to parse line"))
        .collect()
}

// ------------------------------------------------------------
// Delimited inputs
// ------------------------------------------------------------

/// Split each line by a separator into Vec<Vec<&str>>.
pub fn split_lines(input: &str, sep: char) -> Vec<Vec<&str>> {
    input.lines().map(|l| l.split(sep).collect()).collect()
}

/// Loop over lines and split a line and parse each element using FromStr.
pub fn split_parse_lines<T: FromStr>(input: &str, sep: char) -> Vec<Vec<T>>
where
    <T as FromStr>::Err: Debug,
{
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            l.split(sep)
                .filter(|s| !s.is_empty())
                .map(|s| s.trim().parse::<T>().expect("parse error"))
                .collect()
        })
        .collect()
}

/// Single delimiter-separated line of i64 values.
pub fn split_line_i64(line: &str, sep: char) -> Vec<i64> {
    line.trim()
        .split(sep)
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<i64>().expect("invalid integer"))
        .collect()
}

/// Single whitespace-separated line of i64 values.
pub fn whitespace_line_i64(line: &str) -> Vec<i64> {
    line.split_whitespace()
        .map(|s| s.parse::<i64>().expect("invalid integer"))
        .collect()
}

/// Split a line and parse each element using FromStr.
pub fn split_line_parse<T: FromStr>(line: &str, sep: char) -> Vec<T>
where
    <T as FromStr>::Err: Debug,
{
    line.split(sep)
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<T>().expect("parse error"))
        .collect()
}

// ------------------------------------------------------------
// Grids
// ------------------------------------------------------------

/// Grid of characters.
pub fn char_grid(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

/// Grid of single-digit integers.
pub fn digit_grid(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|l| l.chars().map(|c| c as u8 - b'0').collect())
        .collect()
}

/// Grid of integers separated by whitespace.
pub fn int_grid(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

// ------------------------------------------------------------
// Block-based parsing (blank-line separated sections)
// ------------------------------------------------------------

/// Return blocks separated by blank lines, each block as &str.
pub fn blocks(input: &str) -> Vec<&str> {
    input.split("\n\n").collect()
}

/// Blocks as Vec<Vec<&str>> of lines.
pub fn blocks_of_lines(input: &str) -> Vec<Vec<&str>> {
    input
        .split("\n\n")
        .map(|b| {
            b.lines()
                .map(|l| l.trim())
                .filter(|l| !l.is_empty())
                .collect()
        })
        .collect()
}

/// Blocks where each line is parsed..
pub fn parse_blocks<T: FromStr>(input: &str) -> Vec<Vec<T>>
where
    <T as FromStr>::Err: Debug,
{
    input
        .split("\n\n")
        .map(|b| {
            b.lines()
                .filter(|l| !l.is_empty())
                .map(|l| l.trim().parse::<T>().expect("invalid integer"))
                .collect()
        })
        .collect()
}

// ------------------------------------------------------------
// Pair / tuple helpers
// ------------------------------------------------------------

/// Parses an input where coordinates are listed as "x,y" on lines.
pub fn parse_coordinate_pairs<T: FromStr>(input: &str, sep: char) -> Vec<(T, T)>
where
    <T as FromStr>::Err: Debug,
{
    input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(sep).expect("Line must contain a comma");
            (
                x.trim().parse::<T>().expect("Failed to parse X"),
                y.trim().parse::<T>().expect("Failed to parse Y"),
            )
        })
        .collect()
}

/// Split a line `"a,b"` into (a, b) as i64.
pub fn split_i64_pair(line: &str, sep: char) -> (i64, i64) {
    let mut it = line.split(sep);
    let a = it.next().unwrap().parse().unwrap();
    let b = it.next().unwrap().parse().unwrap();
    (a, b)
}

/// Split a line `"x1 y1"` etc. into Vec<i64>.
pub fn split_to_i64(line: &str, sep: char) -> Vec<i64> {
    line.split(sep)
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<i64>().unwrap())
        .collect()
}
