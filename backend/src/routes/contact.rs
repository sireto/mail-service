use axum::{
    routing::{
        get,
        post,
        patch,
        delete
    }, Router };

use crate::handlers::contact::{
    create_contact,
    get_contacts,
    update_contact,
    delete_contact, 
    get_contact_by_id
};

pub fn contact_routes() -> Router {
    Router::new()
        .route("/", get(get_contacts))
        .route("/", post(create_contact))
        .route("/{contactId}", get(get_contact_by_id))
        .route("/{contactId}", patch(update_contact))
        .route("/{contactId}", delete(delete_contact))
}