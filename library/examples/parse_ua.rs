//! Debugging tool: what does a user-agent string resolve to?
//!
//!   parse_ua "<ua string>"          one UA, normalized + raw parser output
//!   parse_ua --stdin                one UA per line, tab-separated raw output
//!   parse_ua --stdin-normalized     one UA per line, family/version output

use polyfill_library::ua::{UA, UserAgent};
use polyfill_library::useragent::useragent;

fn main() {
    let arg = std::env::args()
        .nth(1)
        .expect("usage: parse_ua <ua> | --stdin | --stdin-normalized");
    if arg == "--stdin" {
        for line in std::io::stdin().lines() {
            let ua_string = line.unwrap();
            let [family, major, minor, patch] = useragent(&ua_string);
            println!("{family}\t{major}\t{minor}\t{patch}");
        }
    } else if arg == "--stdin-normalized" {
        for line in std::io::stdin().lines() {
            let ua = UA::new(&line.unwrap());
            println!("{}/{}", ua.get_family(), ua.get_version());
        }
    } else {
        let ua = UA::new(&arg);
        let [family, major, minor, patch] = useragent(&arg);
        println!("normalized: {}/{}", ua.get_family(), ua.get_version());
        println!("raw parser: {family}/{major}.{minor}.{patch}");
    }
}
