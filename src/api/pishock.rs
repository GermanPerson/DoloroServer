

use poem_openapi::Object;
use poem_openapi::{payload::Json, ApiResponse};
use crate::establish_connection;
use diesel::prelude::*;
use crate::PishockDevice;
use crate::OpenApi;
use poem_openapi::payload::PlainText;
use poem::{error::BadRequest, error::InternalServerError, Result};
use poem_openapi::param::Path;
use std::time::Duration;

#[derive(ApiResponse)]
enum PishockListResponse {
    #[oai(status = 200)]
    Ok(Json<PishockDevice>)
}

#[derive(ApiResponse)]
enum PishockCreateResponse {
    #[oai(status = 200)]
    Ok(Json<NewPishock>),
}

#[derive(Object)]
pub struct NewPishock {
    sharecode: String,
}

#[derive(ApiResponse)]
pub enum PishockIDSearchResponse {
    #[oai(status = 200)]
    Ok(Json<PishockDevice>),
    #[oai(status = 404)]
    NotFound(Json<String>),
    #[oai(status = 500)]
    InternalServerError(Json<String>),
}

#[derive(ApiResponse)]
pub enum PishockShockResponse {
    #[oai(status = 200)]
    Ok(PlainText<String>),
    #[oai(status = 400)]
    BadRequest(Json<String>),
    #[oai(status = 404)]
    NotFound(Json<String>),
    #[oai(status = 500)]
    InternalServerError(Json<String>),
}

#[derive(ApiResponse)]
pub enum PishockVibrateResponse {
    #[oai(status = 200)]
    Ok(PlainText<String>),
    #[oai(status = 400)]
    BadRequest(Json<String>),
    #[oai(status = 404)]
    NotFound(Json<String>),
    #[oai(status = 500)]
    InternalServerError(Json<String>),
}

#[derive(Object)]
pub struct PishockVibrateRequest {
    intensity: i32,
    duration: i32,
}

#[derive(Object)]
pub struct PishockShockRequest {
    intensity: i32,
    duration: i32,
    warn: bool
}

pub struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/pishock/list", method = "get")]
    pub async fn pishock_device_list(&self) -> Json<Vec<PishockDevice>> {
        use crate::schema::pishock_devices::dsl::pishock_devices;

        let conn = &mut establish_connection();
    
        let results = pishock_devices.load::<PishockDevice>(conn).expect("DB conn failed");

        Json(results)
    }

    #[oai(path = "/pishock/create", method = "post")]
    pub async fn pishock_device_create(&self, device: Json<NewPishock>) -> Result<Json<PishockDevice>, poem::Error> {
        use crate::schema::pishock_devices::dsl::pishock_devices;

        let conn = &mut establish_connection();

        // Get PiShock credentials from env
        let pishock_username = std::env::var("PISHOCK_USERNAME").map_err(InternalServerError)?;
        let pishock_apikey = std::env::var("PISHOCK_APIKEY").map_err(InternalServerError)?;

        let pishock_acc = pishock_rs::PiShockAccount::new("Doloro Server", pishock_username.as_str(), pishock_apikey.as_str());
        match pishock_acc.get_shocker(device.sharecode.clone()).await {
            Ok(shocker) => {
                println!("Found shocker with name: {}", shocker.get_shocker_name().unwrap());
                println!("Max intensity: {}", shocker.get_max_intensity().unwrap());
                println!("Max duration: {:#?}", shocker.get_max_duration().unwrap());

                // Add PiShock device to database
                let new_pishock = PishockDevice {
                    id: 0,
                    name: shocker.get_shocker_name().unwrap(),
                    sharecode: device.sharecode.clone(),
                    max_intensity: shocker.get_max_intensity().unwrap() as i32,
                    max_duration: shocker.get_max_duration().unwrap().as_secs() as i32,
                };

                diesel::insert_into(pishock_devices)
                    .values(&new_pishock)
                    .execute(conn)
                    .expect("DB conn failed");

                Ok(Json(new_pishock))
            },
            Err(e) => {
                println!("Error: {}", e);

                Err(BadRequest(e))
            }
        }
    }

    #[oai(path = "/pishock/:shockerid", method = "get")]
    pub async fn pishock_device_get(&self, shockerid: Path<i32>) -> Result<PishockIDSearchResponse> {
        use crate::schema::pishock_devices::dsl::pishock_devices;

        let conn = &mut establish_connection();

        println!("Getting PiShock device with id: {}...", shockerid.0);
        let result = pishock_devices.find(shockerid.0).load::<PishockDevice>(conn);

        match result {
            Ok(mut results) => {
                if !results.is_empty() {
                    Ok(PishockIDSearchResponse::Ok(Json(results.pop().unwrap())))
                } else {
                    Ok(PishockIDSearchResponse::NotFound(Json("No device found with that ID".to_string())))
                }
            },
            Err(e) => {
                println!("Error: {}", e);

                Err(InternalServerError(e))
            }
        }
    }

    #[oai(path = "/pishock/:shockerid/shock", method = "post")]
    pub async fn pishock_execute_shock(&self, shockerid: Path<i32>, shockrequest: Json<PishockShockRequest>) -> Result<PishockShockResponse> {
        use crate::schema::pishock_devices::dsl::pishock_devices;

        let conn = &mut establish_connection();

        println!("Getting PiShock device with id: {}...", shockerid.0);
        let pishock_db = pishock_devices.find(shockerid.0).load::<PishockDevice>(conn).map_err(InternalServerError)?;
        if pishock_db.is_empty() {
            return Ok(PishockShockResponse::NotFound(Json("No device found with that ID".to_string())));
        }

        // Get PiShock credentials from env
        let pishock_username = std::env::var("PISHOCK_USERNAME").map_err(InternalServerError)?;
        let pishock_apikey = std::env::var("PISHOCK_APIKEY").map_err(InternalServerError)?;

        let pishock_instance = pishock_rs::PiShockAccount::new("Doloro Server", pishock_username.as_str(), pishock_apikey.as_str());

        let pishocker = pishock_instance.get_shocker(pishock_db[0].sharecode.clone()).await.unwrap();

        let shockresponse = if shockrequest.warn {
            pishocker.shock_with_warning(shockrequest.intensity as u32, Duration::from_millis(shockrequest.duration as u64)).await
        } else {
            pishocker.shock(shockrequest.intensity as u32, Duration::from_millis(shockrequest.duration as u64)).await
        };

        match shockresponse {
            Ok(_) => {
                Ok(PishockShockResponse::Ok(PlainText("Shock executed successfully".to_string())))
            },
            Err(e) => {
                Ok(PishockShockResponse::BadRequest(Json(e.to_string())))
            }
        }
    }

    #[oai(path = "/pishock/:shockerid/vibrate", method = "post")]
    pub async fn pishock_execute_vibrate(&self, shockerid: Path<i32>, vibraterequest: Json<PishockVibrateRequest>) -> Result<PishockVibrateResponse> {
        use crate::schema::pishock_devices::dsl::pishock_devices;

        let conn = &mut establish_connection();

        println!("Getting PiShock device with id: {}...", shockerid.0);
        let pishock_db = pishock_devices.find(shockerid.0).load::<PishockDevice>(conn).map_err(InternalServerError)?;
        if pishock_db.is_empty() {
            return Ok(PishockVibrateResponse::NotFound(Json("No device found with that ID".to_string())));
        }

        // Get PiShock credentials from env
        let pishock_username = std::env::var("PISHOCK_USERNAME").map_err(InternalServerError)?;
        let pishock_apikey = std::env::var("PISHOCK_APIKEY").map_err(InternalServerError)?;

        let pishock_instance = pishock_rs::PiShockAccount::new("Doloro Server", pishock_username.as_str(), pishock_apikey.as_str());

        let pishocker = pishock_instance.get_shocker(pishock_db[0].sharecode.clone()).await.unwrap();

        let vibrateresponse = pishocker.vibrate(vibraterequest.intensity as u32, Duration::from_millis(vibraterequest.duration as u64)).await;

        match vibrateresponse {
            Ok(_) => {
                Ok(PishockVibrateResponse::Ok(PlainText("Vibrate executed successfully".to_string())))
            },
            Err(e) => {
                Ok(PishockVibrateResponse::BadRequest(Json(e.to_string())))
            }
        }
    }
}

