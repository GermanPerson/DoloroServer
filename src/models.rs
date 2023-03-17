

use diesel::prelude::*;
use poem_openapi::{Object};


#[derive(Queryable, Selectable, Object, Insertable, Debug)]
#[diesel(table_name = crate::schema::pishock_devices)]
pub struct PishockDevice {
    pub id: i32,
    pub name: String,
    pub sharecode: String,
    pub max_intensity: i32,
    pub max_duration: i32,
}
