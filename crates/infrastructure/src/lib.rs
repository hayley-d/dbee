//! Infrastructure (innermost): data models and repositories.
//! Depends on nothing else in the workspace.

pub mod models {
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct UserId(pub u64);

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct User {
        pub id: UserId,
        pub name: String,
    }
}

pub mod repositories {
    use super::models::{User, UserId};
    use std::collections::HashMap;
    use std::sync::Mutex;

    #[derive(Default)]
    pub struct UserRepository {
        users: Mutex<HashMap<UserId, User>>,
    }

    impl UserRepository {
        pub fn find(&self, id: &UserId) -> Option<User> {
            self.users.lock().unwrap().get(id).cloned()
        }

        pub fn insert(&self, user: User) {
            self.users.lock().unwrap().insert(user.id.clone(), user);
        }
    }
}

// Convenience re-exports for outer layers.
pub mod interpreter;

pub use models::{User, UserId};
pub use repositories::UserRepository;
