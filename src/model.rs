pub use sherpa_rs::tts::{KokoroTts, KokoroTtsConfig};

pub fn generate() {
    let config = KokoroTtsConfig {
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
    let sid = 0;
    let audio: sherpa_rs::tts::TtsAudio = tts
        .create("Hello! My name is Mohamed Hisham!", sid, 1.0)
        .unwrap();
    sherpa_rs::write_audio_file("audio.wav", &audio.samples, audio.sample_rate).unwrap();
    println!("Created audio.wav")
}
