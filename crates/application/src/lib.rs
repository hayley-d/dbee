use domains::{CreateUser, GetUserName};
use infrastructure::{User, UserRepository};

pub struct UserService<'a> {
    pub repo: &'a UserRepository,
}

impl UserService<'_> {
    pub fn handle_create(&self, cmd: CreateUser) {
        self.repo.insert(User {
            id: cmd.id,
            name: cmd.name,
        });
    }

    pub fn handle_get_name(&self, query: GetUserName) -> Option<String> {
        self.repo.find(&query.id).map(|u| u.name)
    }
}
