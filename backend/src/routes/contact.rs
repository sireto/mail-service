use axum::{
    routing::{delete, get, patch, post},
    Router,
};

use crate::handlers::contact::{
    check_email, create_contacts, delete_contact, get_contact_by_id, get_contacts, get_mails_by_contact_id,
    import_contacts, update_contact,
};

pub fn contact_routes() -> Router {
    Router::new()
        .route("/", get(get_contacts))
        .route("/", post(create_contacts))
        .route("/{contactId}", get(get_contact_by_id))
        .route("/{contactId}", patch(update_contact))
        .route("/{contactId}", delete(delete_contact))
        .route("/check-email", get(check_email))
        .route("/import", post(import_contacts))
        .route("/{contactId}/mails", get(get_mails_by_contact_id))
}
