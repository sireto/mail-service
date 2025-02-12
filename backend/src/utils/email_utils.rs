use email_address::*;
use anyhow::{ Result, anyhow };

pub fn enumerate_list(receiver: String) -> Result<Vec<String>> {
    // Check if the receiver is a comma-separated list of emails...
    let receiver_list: Vec<String> = receiver
    .split(',')
    .map(|r| r.trim().to_string())
    .collect();

    // Validate each email in the list...
    if receiver_list.iter().all(|email| EmailAddress::is_valid(email)) {
        return Ok(receiver_list);
    }

    Err(anyhow!("Invalid email address detected"))
}