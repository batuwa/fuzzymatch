# FuzzyMatch

A simple fuzzy string matching library that provides functionality similar to the `Ctrl-p` command in
Sublime Text. 

The code is very raw right now and was created as a way for the author to learn Rust programming. However,
I hope to keep adding more functionality as I learn more. The initial implementation is a direct port of the algorithm 
described in this [blog](https://blog.forrestthewoods.com/reverse-engineering-sublime-text-s-fuzzy-match-4cffeed33fdb). 

## Features

Given a `string` and a `pattern`, the algorithm finds out whether the string contains all the characters in the pattern signature in order. It uses various heuristics to score the match.

I plan to create a web frontend in the future that will use this fuzzy matcher. 

## Usage

The code is supplied as a library at the moment. It can be added in the Cargo.toml file of your cargo project with the github link as
`fuzzymatch = { git = "https://github.com/batuwa/fuzzymatch.git" }` and importing it into the module it will be used like any other
crate `extern crate fuzzymatch`. See the [fuzzysearch](https://github.com/batuwa/fuzzysearch) repo for an example. 

### Simple Match Algorithm

This function tells if the given fuzzy search pattern can be found in the given string in sequence. It can be used for any sub-string matching use-case.  

```rust
use fuzzymatch::algo::fuzzy_match_simple;

let search = "something";
let my_string = "some search thing";

let matched = fuzzy_match_simple(search, my_string);

println!("Pattern is given string: {:?}", matched);
```

### Scored Match Algorithm

This function provides a score to each matching string based on whether the match occurred in the beginning of the string, beginning of a word, uppercase letter etc. Higher score means better match. It also returns the indices of the matched characters.

```rust
use fuzzymatch::algo::fuzzy_match;

let search = "something";
let my_string1 = "some search thing";
let my_string2 = "this is really something";

let (is_match1, score1, matching_indices1) = fuzzy_match(search, my_string1);
let (is_match2, score2, matching_indices1) = fuzzy_match(search, my_string2);

println!("Score for string1 = {:?}", score1);
println!("Score for string2 = {:?}", score2);
``` 