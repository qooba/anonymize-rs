use actix_files as fs;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_web::middleware::Logger;
use anonymize_rs::anonymizer::{AnonymizePipeline, Anonymizer, ReplaceResult};
use anonymize_rs::config::AnonymizePipelineConfig;
use anyhow::Result;
use bytes::{Bytes, BytesMut};
use clap::{Parser, Subcommand};
use models::{AnonymizeRequest, DeAnonymizeResponse};
use once_cell::sync::OnceCell;
use std::collections::HashMap;
use std::error::Error;
use std::io::{self, BufRead};
use std::path::PathBuf;
use std::sync::mpsc::{sync_channel, SyncSender};
use std::sync::Arc;
use std::thread;
use tokio::sync::mpsc::{channel, Receiver};
use tokio::sync::Mutex;

pub mod anonymizer;
pub mod config;
pub mod error;
pub mod models;

#[derive(Parser)] // requires `derive` feature
#[command(name = "cargo")]
#[command(bin_name = "cargo")]
enum AnonymizeCli {
    Server(ServerArgs),
    File(FileArgs),
    Stdin(StdinArgs),
}

#[derive(Debug, clap::Args)]
struct FileArgs {
    #[arg(long, short = 'c')]
    config: String,

    #[arg(long, short = 'i')]
    input_file: String,

    #[arg(long, short = 'o')]
    output_file: String,
}

#[derive(Debug, clap::Args)]
struct StdinArgs {
    #[arg(long, short = 'c')]
    config: String,
}

#[derive(clap::Args)]
#[command(author, version, about, long_about = None)]
struct ServerArgs {
    #[arg(long, short = 'c')]
    config: String,
    #[arg(long, short = 'b')]
    host: String,
    #[arg(long, short = 'p')]
    port: u16,
}

pub async fn health() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

pub async fn anonymize_post(
    anonymize_request: web::Json<AnonymizeRequest>,
    anonymizer_pipeline: web::Data<AnonymizePipeline>,
) -> Result<impl Responder, Box<dyn Error>> {
    let resp = anonymizer_pipeline.anonymize(&anonymize_request.text, None)?;
    Ok(web::Json(resp))
}

pub async fn anonymize_get(
    anonymize_request: web::Query<AnonymizeRequest>,
    anonymizer_pipeline: web::Data<AnonymizePipeline>,
) -> Result<impl Responder, Box<dyn Error>> {
    let resp = anonymizer_pipeline.anonymize(&anonymize_request.text, None)?;
    Ok(web::Json(resp))
}

pub async fn deanonymize(
    anonymize_request: web::Json<ReplaceResult>,
) -> Result<impl Responder, Box<dyn Error>> {
    let resp = DeAnonymizeResponse {
        text: "".to_string(),
    };
    Ok(web::Json(resp))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = AnonymizeCli::parse();

    match args {
        AnonymizeCli::Server(server_args) => {
            let host = server_args.host.to_string();
            let port: u16 = server_args.port;

            let anonymize_config = AnonymizePipelineConfig::new(&server_args.config)
                .await
                .unwrap();

            let log_level = "debug";
            env_logger::init_from_env(env_logger::Env::new().default_filter_or(log_level));
            HttpServer::new(move || {
                App::new()
                    .app_data(web::Data::new(
                        AnonymizePipeline::new(anonymize_config.clone()).unwrap(),
                    ))
                    .route("/api/anonymize", web::post().to(anonymize_post))
                    .route("/api/anonymize", web::get().to(anonymize_get))
                    .route("/api/deanonymize", web::post().to(deanonymize))
                    .wrap(Logger::default())
            })
            .bind((host, port))?
            .run()
            .await
        }
        AnonymizeCli::File(file_args) => Ok(()),
        AnonymizeCli::Stdin(stdin_args) => {
            let stdin = io::stdin();
            let mut lines = stdin.lock().lines();

            while let Some(line) = lines.next() {
                let length: i32 = line.unwrap().trim().parse().unwrap();

                for _ in 0..length {
                    let line = lines
                        .next()
                        .expect("there was no next line")
                        .expect("the line could not be read");

                    println!("{}", line);
                }
            }

            Ok(())
        }
    }
}
