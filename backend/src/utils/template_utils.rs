use crate::utils::email_utils;

pub fn merge_receipients (r1: String, r2: String, r3: String) -> Vec<String> {
    let receivers = email_utils::enumerate_list(r1);
    let ccs = email_utils::enumerate_list(r2);
    let bccs = email_utils::enumerate_list(r3);

    let mut receipients = Vec::new();
    receipients.extend(receivers.into_iter().flatten());
    receipients.extend(ccs.into_iter().flatten());
    receipients.extend(bccs.into_iter().flatten());

    println!("Receipients: {receipients:?}");
    receipients
}