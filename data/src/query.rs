use ::entities::{
    device, device::Entity as Device,
    device_interactions, device_interactions::Entity as DeviceInteractions
};
use sea_orm::*;

pub struct DeviceQuery;

impl DeviceQuery {
    pub async fn list_devices(db: &DbConn) -> Result<Vec<device::Model>, DbErr> {
        Device::find().all(db).await
    }
}

pub struct DeviceInteractionsQuery;

impl DeviceInteractionsQuery {
    pub async fn list_device_interactions(db: &DbConn, dev_id: i32) -> Result<Vec<device_interactions::Model>, DbErr> {
        DeviceInteractions::find()
            .filter(device_interactions::Column::DeviceId.eq(dev_id))
            .all(db).await
    }
}