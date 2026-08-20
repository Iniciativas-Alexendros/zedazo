#![no_main]

use libfuzzer_sys::fuzz_target;
use zedazo::infrastructure::parser::{parse_vcards, unfold};

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let unfolded = unfold(s);
        let _ = parse_vcards(&unfolded);
    }
});