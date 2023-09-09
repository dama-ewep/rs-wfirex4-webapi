use std::env;
use std::fs;
use std::path::Path;
use std::str::FromStr;

use log::{error, info, LevelFilter};

use actix_web::middleware::Logger;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

use config::{Config, File};
use serde::Deserialize;
use serde_json::json;
mod rs_wfirex4_api;

async fn heartbeat() -> impl Responder {
    HttpResponse::Ok().json(json!({"status": "OK"}))
}

#[derive(Debug, Deserialize)]
struct Configuration {
    log: LogSettings,
    app: AppSettings,
    rs_wfirex4_api: rs_wfirex4_api::Settings,
}
#[derive(Debug, Deserialize)]
struct LogSettings {
    log_file: String,
    log_level: String,
}
#[derive(Debug, Deserialize)]
struct AppSettings {
    service_port: u16,
    service_host: String,
}

fn get_config(filename: &str) -> Configuration {
    let mut settings_builder = Config::builder()
        .set_default("log.log_file", "rs-wfirex4.log")
        .unwrap()
        .set_default("log.log_level", "info")
        .unwrap()
        .set_default("rs_wfirex4_api.host", "rs-wfirex4")
        .unwrap()
        .set_default("app.service_port", 8080)
        .unwrap()
        .set_default("app.service_host", "0.0.0.0")
        .unwrap();
    if Path::new(filename).exists() {
        settings_builder = settings_builder.add_source(File::with_name(filename));
    } else {
        println!("Warning: '{}' not found. Using default values.", filename);
    }

    let settings = settings_builder.build().unwrap();

    let conf: Configuration = settings.try_deserialize().unwrap_or_else(|e| {
        panic!("Failed to parse config: {:?}", e);
    });
    conf
}

fn log_setting(config: &Configuration) -> std::io::Result<()> {
    let file_path = &config.log.log_file;

    if let Some(parent_dir) = Path::new(file_path).parent() {
        // ディレクトリを再帰的に作成
        fs::create_dir_all(parent_dir)?;
    }

    // log出力レベル設定
    let log_level = LevelFilter::from_str(&config.log.log_level).unwrap_or_else(|_| {
        println!("Failed to parse log level, using default level: Info");
        LevelFilter::Info
    });

    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(log_level)
        .chain(std::io::stdout())
        .chain(fern::log_file(file_path)?)
        .apply()
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;

    Ok(())
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // 引数解釈
    let args: Vec<String> = env::args().collect();
    let config_file = if args.len() > 1 {
        &args[1]
    } else {
        "config.toml"
    };

    // 設定ファイル読み込み
    let conf: Configuration = get_config(config_file);

    // ログ設定
    log_setting(&conf)?;
    info!("Starting the application...");

    match HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .route("/heartbeat", web::get().to(heartbeat))
            .service(
                web::scope("/rs-wfirex4/v1")
                    .configure(|cfg| rs_wfirex4_api::get_route(cfg, &conf.rs_wfirex4_api)),
            )
    })
    .bind(format!(
        "{}:{}",
        conf.app.service_host, conf.app.service_port
    )) {
        Ok(server) => server.run().await,
        Err(e) => {
            error!("Failed to start the server: {}", e);
            Ok(())
        }
    }
}
