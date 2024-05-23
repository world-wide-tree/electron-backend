use std::sync::Arc;

use super::{device::DeviceService, home::HomeService, room::RoomService, substation::SubstationService, user::UserService};

#[derive(Clone)]
pub struct AppState{
    user: Arc<UserService>,
    substation: Arc<SubstationService>,
    device: Arc<DeviceService>,
    room: Arc<RoomService>,
    home: Arc<HomeService>,
}

impl AppState{
    pub fn init() -> Self {
        Self { 
            user: Arc::new(UserService::new()),
            substation: Arc::new(SubstationService::new()),
            device: Arc::new(DeviceService::new()),
            room: Arc::new(RoomService::new()),
            home: Arc::new(HomeService::new()),
        }
    }
    pub fn user(&self) -> Arc<UserService>{
        let rst = self.user.clone();
        rst
    }
    pub fn home(&self) -> Arc<HomeService>{
        let rst = self.home.clone();
        rst
    }
    pub fn device(&self) -> Arc<DeviceService>{
        let rst = self.device.clone();
        rst
    }
    pub fn room(&self) -> Arc<RoomService>{
        let rst = self.room.clone();
        rst
    }
    pub fn substation(&self) -> Arc<SubstationService>{
        let rst = self.substation.clone();
        rst
    }
}