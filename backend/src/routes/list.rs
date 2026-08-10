use axum::{
    routing::{delete, get, patch, post},
    Router,
};

use crate::handlers::list::{
    add_contacts_to_list, create_list, delete_list, get_contacts_from_lists, get_list_by_id, get_lists,
    remove_contacts_from_list, update_list,
};

pub fn list_routes() -> Router {
    Router::new()
        .route("/", post(create_list))
        .route("/namespaces/{namespace_id}/list", get(get_lists))
        .route("/namespaces/{namespace_id}/list/{list_id}", get(get_list_by_id))
        .route("/namespaces/{namespace_id}/list/{list_id}", patch(update_list))
        .route("/namespaces/{namespace_id}/list/{list_id}", delete(delete_list))
        .route("/addContacts/{list_id}", post(add_contacts_to_list))
        .route("/removeContacts/{list_id}", delete(remove_contacts_from_list))
        .route("/contacts", post(get_contacts_from_lists))
}
