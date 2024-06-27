mod time_labels;

use wasm_bindgen::prelude::*;
use js_sys::Date;
use time_labels::{Language, TIME_LABELS};



#[wasm_bindgen]
pub fn time_ago(timestamp: f64, detailed_label: bool, lang: Language) -> String {
    let now = Date::now();
    let diff_in_seconds = ((now - timestamp) / 1000.0) as i64;

    let intervals = TIME_LABELS.get(&lang).unwrap();

    if detailed_label {
        let mut remaining_seconds = diff_in_seconds;
        let mut parts = vec![];

        for (index, (singular, plural)) in intervals.iter().enumerate() {
            let seconds = match index {
                0 => 31536000, // year
                1 => 2592000,  // month
                2 => 86400,    // day
                3 => 3600,     // hour
                4 => 60,       // minute
                _ => 1,        // second
            };

            let count = remaining_seconds / seconds;
            if count > 0 {
                remaining_seconds %= seconds;
                parts.push(format!("{} {} ", count, if count != 1 { plural } else { singular }));
            }
        }

        if parts.is_empty() {
            "just now".to_string()
        } else {
            format!("{} ago", parts.join(", "))
        }
    } else {
        for (index, (singular, plural)) in intervals.iter().enumerate() {
            let seconds = match index {
                0 => 31536000, // year
                1 => 2592000,  // month
                2 => 86400,    // day
                3 => 3600,     // hour
                4 => 60,       // minute
                _ => 1,        // second
            };

            let count = diff_in_seconds / seconds;
            if count > 0 {
                return format!("{} {} {}", count, if count != 1 { plural } else { singular }, "ago");
            }
        }

        "just now".to_string()
    }
}


// mod time_labels;

// use wasm_bindgen::prelude::*;
// use js_sys::Date;

// #[wasm_bindgen]
// pub fn time_ago(timestamp: f64, detailed_label: bool) -> String {
//     let now = Date::now();
//     let diff_in_seconds = ((now - timestamp) / 1000.0) as i64;

//     let intervals = [
//         ("year", 31536000),
//         ("month", 2592000),
//         ("day", 86400),
//         ("hour", 3600),
//         ("minute", 60),
//         ("second", 1),
//     ];

//     if detailed_label {
//         let mut remaining_seconds = diff_in_seconds;
//         let mut parts = vec![];

//         for (label, seconds) in intervals.iter() {
//             let count = remaining_seconds / seconds;
//             if count > 0 {
//                 remaining_seconds %= seconds;
//                 parts.push(format!("{} {}{}", count, label, if count != 1 { "s" } else { "" }));
//             }
//         }

//         if parts.is_empty() {
//             "just now".to_string()
//         } else {
//             format!("{} ago", parts.join(", "))
//         }
//     } else {
//         for (label, seconds) in intervals.iter() {
//             let count = diff_in_seconds / seconds;
//             if count > 0 {
//                 return format!("{} {}{} ago", count, label, if count != 1 { "s" } else { "" });
//             }
//         }

//         "just now".to_string()
//     }
// }
