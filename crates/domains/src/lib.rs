use infrastructure::UserId;

#[derive(Debug, Clone)]
pub struct CreateUser {
    pub id: UserId,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct GetUserName {
    pub id: UserId,
}
