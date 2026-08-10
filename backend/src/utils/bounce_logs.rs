use crate::models::bounce_logs::Message;

pub fn search_header_from_header_list(message: &Message, header_name: &str) -> Option<String> {
    let message_id = message
        .mail
        .headers
        .iter()
        .find(|header| header.name.eq_ignore_ascii_case(header_name))
        .map(|header| header.value.clone());

    message_id
}
