use polyfill_library::ua::{UserAgent, UA};
fn main() {
    let ua_string = std::env::args().nth(1).unwrap();
    let ua = UA::new(&ua_string);
    println!("{}/{}", ua.get_family(), ua.get_version());
}
