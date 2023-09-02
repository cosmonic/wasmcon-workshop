use anyhow::{anyhow, Context};

use crate::wasi::{
    io::streams::write,
    keyvalue::{
        readwrite::{get, set},
        types::{
            incoming_value_consume_sync, new_outgoing_value, outgoing_value_write_body,
        },
    },
    logging::logging::{log, Level},
};

/// Increment (possibly negatively) the counter for a given key
pub fn increment_counter(bucket: u32, key: &String, amount: i32) -> anyhow::Result<i32> {
    let current_value: i32 = match get(bucket, key) {
        // If the value exists, parse it into an i32
        Ok(incoming_value) => {
            // Read bytes from incoming value
            let bytes = incoming_value_consume_sync(incoming_value)
                .map_err(|count| anyhow!("failed to parse incoming bytes, read [{count}]"))?;
            // Convert the bytes to a i32
            String::from_utf8(bytes)
                .context("failed to parse string from returned bytes")?
                .trim()
                .parse()
                .context("failed to parse i32 from bytes")?
        }
        // If the value is missing or we fail to get it, assume it is zero
        Err(_) => {
            eprintln!("[warn] encountered missing key [{key}], defaulting to 0");
            0
        }
    };

    log(Level::Info, "rust-component", format!("current value: {current_value}").as_str());

    // Calculate the new value
    let new_value: i32 = current_value + amount;

    // Build outgoing value to use
    let outgoing_value = new_outgoing_value();
    let stream =
        outgoing_value_write_body(outgoing_value).expect("failed to write outgoing value");

    // Write out the new value
    write(stream, new_value.to_string().as_bytes())
        .expect("failed to write to outgoing value stream");

    // Set the key to the updated value
    set(bucket, key, outgoing_value).expect("failed to set value");

    // Cheat and just assume the new value will be the increment
    Ok(new_value)
}