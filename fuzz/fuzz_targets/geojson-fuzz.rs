#![no_main]
use libfuzzer_sys::fuzz_target;
use geojson::GeoJson;
use std::str;

fuzz_target!(|data: &[u8]| {
    match str::from_utf8(data) {
        Ok(in_string)=>{
            in_string.parse::<GeoJson>();
        },
        Err(..)=>()
    }
});
