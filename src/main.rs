mod audio_reader;
mod audio_result;

use crate::audio_reader::AudioReader;
use crate::audio_result::AudioResult;
use walkdir::WalkDir;

// config
const IN_DIR: &str = r"D:\OneDrive - Lake Washington School District\Everything Else\gay\sound is gay\vgm server (ie parker)";
pub const CHANNELS: u16 = 2;
pub const SAMPLE_RATE: u32 = 44100;
pub const OUT_FILE: &str = r".\bruh.wav";

fn main() {
    // read all paths recursively, ignoring errors
    let paths = WalkDir::new(IN_DIR)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.into_path())
        .filter(|path| path.is_file())
        .collect::<Vec<_>>();

    let mut count: usize = 1;
    let total = paths.len();

    paths
        .iter()
        .fold(AudioResult::new(), |mut audio_result, path| {
            println!("{}/{} - {:?}", count, total, path);
            count += 1;

            let reader = AudioReader::open(&path);
            if reader.is_err() {
                println!("error: {:?}", reader.err().unwrap());
                return audio_result;
            }
            let reader = reader.unwrap();

            for (index, sample) in reader.enumerate() {
                audio_result.add(index, sample);
            }

            audio_result
        })
        .save();

    println!("done!");
    std::thread::sleep(std::time::Duration::from_secs(10));
}
