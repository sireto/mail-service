// the following functions are not currently used it might be used for the cleanup process for the contact list while developing...

use aws_sdk_sesv2::{ Client, Error };

pub async fn delete_contact_from_list(client: &Client, list_name: &str, email: &str) -> Result<(), Error> {
    // Attempt to delete the contact from the specified contact list

    match client
        .delete_contact()
        .contact_list_name(list_name)
        .email_address(email)
        .send()
        .await
    {
        Ok(_) => {
            println!("Contact {} successfully deleted from list {}", email, list_name);
            Ok(())
        }
        Err(err) => {
            eprintln!(
                "Failed to delete contact {} from list {}: {}",
                email, list_name, err
            );
            Err(err.into())
        }
    }
}


async fn show_contacts(client: &Client, list: &str) -> Result<(), Error> {
    let resp = client
        .list_contacts()
        .contact_list_name(list)
        .send()
        .await?;

    println!("Contacts:");

    for contact in resp.contacts() {
        println!("  {}", contact.email_address().unwrap_or_default());
    }

    Ok(())
}

