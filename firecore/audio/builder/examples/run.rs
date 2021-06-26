fn main() {
    let start = std::time::Instant::now();
    audio_builder::compile("music", "output/audio.bin");
    println!("Finished in {}ms.", start.elapsed().as_millis());
}