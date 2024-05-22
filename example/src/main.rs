use std::path::PathBuf;

use geng_web_audio_api::{self as audio, AudioNode as _};

// async because dont want to block on web
async fn run() {
    let audio = audio::AudioContext::new().unwrap();

    audio.listener().set_position([0.0, 0.0, 0.0]);
    audio
        .listener()
        .set_orientation([0.0, 0.0, -1.0], [0.0, 1.0, 0.0]);

    // If not running with `cargo run --package example`
    // audio file is expected in current working dir
    let root_dir: PathBuf = match std::env::var("CARGO_MANIFEST_DIR") {
        Ok(dir) => dir.into(),
        Err(_) => ".".into(),
    };

    // std::fs::read_file would block
    let bytes = batbox_file::load_bytes(root_dir.join("audio.wav"))
        .await
        .expect("Failed to load audio file");
    let buffer = audio
        .decode(bytes)
        .await
        .expect("Failed to parse audio file");
    let play_sound = move || {
        let mut source = audio::AudioBufferSourceNode::new(&audio);
        source.set_buffer(buffer.clone());
        let mut panner = audio::PannerNode::new(&audio);
        // should be in right ear
        panner.set_position([1.0, 0.0, 0.0]);
        source.connect(&panner);
        panner.connect(&audio.destination());
        source.start_with_offset(0.0);
    };
    #[cfg(not(target_arch = "wasm32"))]
    play_sound();
    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::JsCast as _;
        let play_sound = Box::new(play_sound) as Box<dyn Fn()>;
        let play_sound = wasm_bindgen::closure::Closure::wrap(play_sound)
            .into_js_value()
            .unchecked_into();
        web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .set_onclick(Some(&play_sound));
    }

    // std::time::sleep would block
    batbox_time::sleep(batbox_time::Duration::from_secs_f64(1.0)).await;
}

fn main() {
    #[cfg(target_arch = "wasm32")]
    {
        console_error_panic_hook::set_once();
        wasm_bindgen_futures::spawn_local(run());
    }
    #[cfg(not(target_arch = "wasm32"))]
    futures::executor::block_on(run());
}
