use std::sync::Arc;

use super::user::UserService;

#[derive(Clone)]
pub struct AppState{
    user: Arc<UserService>
}

impl AppState{
    pub fn init() -> Self {
        Self { user: Arc::new(UserService::new()) }
    }
    pub fn user(&self) -> Arc<UserService>{
        let rst = self.user.clone();
        rst
    }
}