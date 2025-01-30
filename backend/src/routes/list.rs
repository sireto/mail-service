use axum::{
    routing::{
        get,
        post,
        patch,
        delete
    }, Router };

use crate::handlers::list::{
    create_list,
    get_lists,
    update_list,
    delete_list, 
    get_list_by_id,
    add_contacts_to_list,
    remove_contacts_from_list
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
}