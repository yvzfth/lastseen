mod time_labels;

use wasm_bindgen::prelude::*;
use js_sys::Date;
use time_labels::{Language, TIME_LABELS};

/// Calculates the time ago from a given timestamp or `Date` object.
///
/// # Arguments
///
/// * `input` - The timestamp or `Date` object representing the time to calculate the time ago from.
/// * `detailed_label` - An optional boolean value indicating whether to include detailed labels (e.g., "2 hours, 30 minutes ago") or not. Defaults to `false`.
/// * `lang` - An optional `Language` enum value indicating the language to use for the labels. Defaults to the default language.
///
/// # Returns
///
/// A `String` representing the time ago from the given timestamp or `Date` object.
///
/// # Panics
///
/// This function will panic if the `input` is not a valid `Date` object or timestamp.
///
/// # Example
///
/// ```
/// import React, { useState, useEffect } from 'react';
/// import init, { time_ago, Language } from 'timeagoplus';
/// 
/// const TimeAgoComponent: React.FC = () => {
///     const [timeAgoString, setTimeAgoString] = useState<string>('');
/// 
///     useEffect(() => {
///         const fetchData = async () => {
///             await init();
///             const timestamp = Date.now();
///             const result = time_ago(timestamp, true, Language.English);
///             setTimeAgoString(result);
///         };
/// 
///         fetchData();
///     }, []);
/// 
///     return (
///         <div>
///             Time ago: {timeAgoString}
///         </div>
///     );
/// };
/// 
/// export default TimeAgoComponent;
/// ```
#[wasm_bindgen]
pub fn time_ago(input: JsValue, detailed_label: Option<bool>, lang: Option<Language>) -> String {
  
    let timestamp = if input.is_instance_of::<Date>() {
        input.dyn_into::<Date>().unwrap().get_time()
    } else if input.as_f64().map(|f| f.is_finite()).unwrap_or(false) {
        input.as_f64().unwrap()
    } else {
        panic!("Invalid input: expected a Date object or a timestamp.");
    };

    let now = Date::now();
    let diff_in_seconds = ((now - timestamp) / 1000.0) as i64;

    // Set default values for the optional parameters
    let detailed_label = detailed_label.unwrap_or(false);
    let lang = lang.unwrap_or(Language::default());

    let (intervals, ago_label) = TIME_LABELS.get(&lang).unwrap(); // Get labels and "ago" string

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
                parts.push(format!("{} {}", count, if count != 1 { plural } else { singular }));
            }
        }

        if parts.is_empty() {
            "just now".to_string()
        } else {
            format!("{} {}", parts.join(", "), ago_label)  // Use ago_label here
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
                return format!("{} {} {}", count, if count != 1 { plural } else { singular }, ago_label); // Use ago_label here
            }
        }

        "just now".to_string()
    }
}


// mod time_labels;

// use wasm_bindgen::prelude::*;
// use js_sys::Date;
// use time_labels::{Language, TIME_LABELS};



// #[wasm_bindgen]
// pub fn time_ago(timestamp: f64, detailed_label: bool, lang: Language) -> String {
//     let now = Date::now();
//     let diff_in_seconds = ((now - timestamp) / 1000.0) as i64;

//     let (intervals, ago_label) = TIME_LABELS.get(&lang).unwrap(); // Get labels and "ago" string

//     if detailed_label {
//         let mut remaining_seconds = diff_in_seconds;
//         let mut parts = vec![];

//         for (index, (singular, plural)) in intervals.iter().enumerate() {
//             let seconds = match index {
//                 0 => 31536000, // year
//                 1 => 2592000,  // month
//                 2 => 86400,    // day
//                 3 => 3600,     // hour
//                 4 => 60,       // minute
//                 _ => 1,        // second
//             };

//             let count = remaining_seconds / seconds;
//             if count > 0 {
//                 remaining_seconds %= seconds;
//                 parts.push(format!("{} {}", count, if count != 1 { plural } else { singular }));
//             }
//         }

//         if parts.is_empty() {
//             "just now".to_string()
//         } else {
//             format!("{} {}", parts.join(", "), ago_label)  // Use ago_label here
//         }
//     } else {
//         for (index, (singular, plural)) in intervals.iter().enumerate() {
//             let seconds = match index {
//                 0 => 31536000, // year
//                 1 => 2592000,  // month
//                 2 => 86400,    // day
//                 3 => 3600,     // hour
//                 4 => 60,       // minute
//                 _ => 1,        // second
//             };

//             let count = diff_in_seconds / seconds;
//             if count > 0 {
//                 return format!("{} {}{}", count, if count != 1 { plural } else { singular }, ago_label); // Use ago_label here
//             }
//         }

//         "just now".to_string()
//     }
// }

// mod time_labels;

// use wasm_bindgen::prelude::*;
// use js_sys::Date;
// use time_labels::{Language, TIME_LABELS};

// #[wasm_bindgen]
// pub fn time_ago(timestamp: f64, detailed_label: Option<bool>, lang: Option<Language>) -> String {
//     let now = Date::now();
//     let diff_in_seconds = ((now - timestamp) / 1000.0) as i64;

//     // Set default values for the optional parameters
//     let detailed_label = detailed_label.unwrap_or(false);
//     let lang = lang.unwrap_or(Language::default());

//     let (intervals, ago_label) = TIME_LABELS.get(&lang).unwrap(); // Get labels and "ago" string

//     if detailed_label {
//         let mut remaining_seconds = diff_in_seconds;
//         let mut parts = vec![];

//         for (index, (singular, plural)) in intervals.iter().enumerate() {
//             let seconds = match index {
//                 0 => 31536000, // year
//                 1 => 2592000,  // month
//                 2 => 86400,    // day
//                 3 => 3600,     // hour
//                 4 => 60,       // minute
//                 _ => 1,        // second
//             };

//             let count = remaining_seconds / seconds;
//             if count > 0 {
//                 remaining_seconds %= seconds;
//                 parts.push(format!("{} {}", count, if count != 1 { plural } else { singular }));
//             }
//         }

//         if parts.is_empty() {
//             "just now".to_string()
//         } else {
//             format!("{} {}", parts.join(", "), ago_label)  // Use ago_label here
//         }
//     } else {
//         for (index, (singular, plural)) in intervals.iter().enumerate() {
//             let seconds = match index {
//                 0 => 31536000, // year
//                 1 => 2592000,  // month
//                 2 => 86400,    // day
//                 3 => 3600,     // hour
//                 4 => 60,       // minute
//                 _ => 1,        // second
//             };

//             let count = diff_in_seconds / seconds;
//             if count > 0 {
//                 return format!("{} {} {}", count, if count != 1 { plural } else { singular }, ago_label); // Use ago_label here
//             }
//         }

//         "just now".to_string()
//     }
// }
