fn main() {
    let start = std::time::Instant::now();
    let music = firecore_audio_builder::compile("test");
    let sound = std::collections::HashMap::<firecore_audio_builder::SoundId, Vec<u8>>::new();
    let data = (music, sound);
    std::fs::write("target/audio.bin", firecore_storage::to_bytes(&data).unwrap()).unwrap();
    println!("Finished in {}ms.", start.elapsed().as_millis());
}