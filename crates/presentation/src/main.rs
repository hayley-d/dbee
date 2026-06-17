//! Presentation: routes / controllers + composition root (the binary).
//! Knows every layer so it can wire concrete repositories into services.

use application::UserService;
use domains::{CreateUser, GetUserName};
use infrastructure::{UserId, UserRepository};

/// A "controller" — in a real app this would be an axum/actix handler.
fn get_user_name_controller(service: &UserService, id: u64) -> String {
    match service.handle_get_name(GetUserName { id: UserId(id) }) {
        Some(name) => format!("200 OK: {name}"),
        None => "404 Not Found".to_string(),
    }
}

fn main() {
    // Composition root: pick the concrete repository and inject it.
    let repo = UserRepository::default();
    let service = UserService { repo: &repo };

    service.handle_create(CreateUser {
        id: UserId(1),
        name: "Ada".into(),
    });

    // "Route" dispatch.
    println!("{}", get_user_name_controller(&service, 1));
    println!("{}", get_user_name_controller(&service, 2));
}
