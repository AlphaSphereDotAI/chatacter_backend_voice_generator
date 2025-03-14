use actix_files::NamedFile;
use actix_multipart::form::tempfile::TempFileConfig;
use actix_web::{
    get, http::header::ContentType, middleware::Logger, post, web, App, HttpResponse, HttpServer,
    Responder,
};
use serde::Deserialize;
use sherpa_rs::tts::{KokoroTts, KokoroTtsConfig, TtsAudio};
use sherpa_rs::write_audio_file;
use uuid::Uuid;

#[derive(Deserialize)]
struct GenerateParams {
    text: String,
    speaker_id: i32,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body("Welcome to the Chatacter's Voice Generator API")
}

#[post("/generate")]
async fn generate(params: web::Json<GenerateParams>) -> impl Responder {
    if params.text.trim().is_empty() {
        return Err(actix_web::error::ErrorBadRequest("Text cannot be empty"));
    };

    if !(0..=10).contains(&params.speaker_id) {
        return Err(actix_web::error::ErrorBadRequest(
            "Speaker ID must be between 0 and 10",
        ));
    };
    log::info!(
        "Generating audio for text: {}, with voice id: {}",
        params.text,
        params.speaker_id
    );
    let config: KokoroTtsConfig = KokoroTtsConfig {
        model: "./kokoro-en-v0_19/model.onnx".to_string(),
        voices: "./kokoro-en-v0_19/voices.bin".into(),
        tokens: "./kokoro-en-v0_19/tokens.txt".into(),
        data_dir: "./kokoro-en-v0_19/espeak-ng-data".into(),
        length_scale: 1.0,
        ..Default::default()
    };
    let mut tts = KokoroTts::new(config);
    // 0->af, 1->af_bella, 2->af_nicole, 3->af_sarah, 4->af_sky, 5->am_adam
    // 6->am_michael, 7->bf_emma, 8->bf_isabella, 9->bm_george, 10->bm_lewis
    log::info!("Generating audio");
    let audio: TtsAudio = tts
        .create(&params.text, params.speaker_id, 1.0)
        .map_err(actix_web::error::ErrorInternalServerError)?;
    log::info!("Writing audio file");
    let filename = format!("./tmp/{}.wav", Uuid::new_v4());
    if let Err(e) = write_audio_file(&filename, &audio.samples, audio.sample_rate) {
        log::info!("Error writing audio file: {:?}", e);
        return Err(actix_web::error::ErrorInternalServerError(format!(
            "Error writing audio file: {:?}",
            e
        )));
    }
    Ok(NamedFile::open_async(&filename).await?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    std::fs::create_dir_all("./tmp")?;
    log::info!("starting HTTP server at http://localhost:8001");
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(generate)
            .wrap(Logger::default())
            .app_data(TempFileConfig::default().directory("./tmp"))
    })
    .bind(("0.0.0.0", 8001))?
    .run()
    .await
}
