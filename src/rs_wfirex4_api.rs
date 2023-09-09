use actix_web::{web, HttpResponse, Responder};
use log::{error, warn};
use serde::Deserialize;
use std::collections::HashMap;
mod devices;
mod raw_api;

#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
    host: String,
}

fn get_ir_data(device_name: &str, button_name: &str) -> Result<&'static [u8], HttpResponse> {
    let device = match devices::DEVICES.get(device_name) {
        Some(v) => v,
        None => {
            let msg = format!("Invalid device '{}'.", device_name);
            warn!("{}", msg);
            return Err(HttpResponse::NotFound().body(msg));
        }
    };
    match device.get(button_name) {
        Some(v) => return Ok(v),
        None => {
            let msg = format!(
                "Invalid button '{}' on device '{}'.",
                button_name, device_name
            );
            warn!("{}", msg);
            return Err(HttpResponse::NotFound().body(msg));
        }
    }
}
async fn packet_output(info: web::Path<(String, String)>) -> impl Responder {
    let device_name = &info.0;
    let button_name = &info.1;

    let button_ir = match get_ir_data(device_name, button_name) {
        Ok(v) => v,
        Err(e) => {
            return e;
        }
    };
    // ここで device と button に基づいた処理を行う
    let payload = raw_api::get_payload(&(*button_ir).to_vec());
    let payload_str: String = payload.iter().map(|byte| format!("{:02x}", byte)).collect();
    return HttpResponse::Ok().body(format!(
        "Button '{}' on device '{}'.\n{}",
        button_name, device_name, payload_str
    ));
}

async fn execute_button(
    info: web::Path<(String, String)>,
    settings: actix_web::web::Data<Settings>,
) -> impl Responder {
    let device_name = &info.0;
    let button_name = &info.1;

    let button_ir = match get_ir_data(device_name, button_name) {
        Ok(v) => v,
        Err(e) => {
            return e;
        }
    }; // ここで device と button に基づいた処理を行う
    match raw_api::send_ir_data(
        &format!("{}:{}", settings.host, 60001),
        &(*button_ir).to_vec(),
    ) {
        Ok(_v) => {
            return HttpResponse::Ok().body(format!(
                "Button '{}' on device '{}' has been pressed.",
                button_name, device_name
            ));
        }
        Err(_e) => {
            let msg = format!(
                "Button '{}' on device '{}' was Error.",
                button_name, device_name
            );
            error!("{}", msg);
            return HttpResponse::InternalServerError().body(msg);
        }
    };
}

async fn list_devices() -> impl Responder {
    let mut data = HashMap::new();
    for (remocon_key, remocon_value) in devices::DEVICES.entries() {
        let buttons: Vec<&&str> = remocon_value.keys().collect();

        data.insert(remocon_key, buttons);
    }
    HttpResponse::Ok().json(data)
}

pub fn get_route(cfg: &mut web::ServiceConfig, settings: &Settings) {
    cfg.app_data(web::Data::new(settings.clone()))
        .route(
            "/packet/{deviceName}/buttons/{buttonName}",
            web::get().to(packet_output),
        )
        .route(
            "/devices/{deviceName}/buttons/{buttonName}",
            web::get().to(execute_button),
        )
        .route("/devices", web::get().to(list_devices));
}
