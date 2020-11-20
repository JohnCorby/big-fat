use walkdir::WalkDir;

/// hardcoded cuz lmao
const PATH: &str = r"D:\OneDrive - Lake Washington School District\Everything Else\gay\sound is gay\vgm server (ie parker)";

const SUPPORTED_FILES: [&str; 3] = ["mp3", "ogg", "wav"];

fn main() {
    // read all paths recursively, ignoring errors
    let path_iter = WalkDir::new(PATH)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.into_path())
        .filter(|path| {
            let ext = path
                .extension()
                .unwrap_or_default()
                .to_str()
                .unwrap_or_default();
            SUPPORTED_FILES.contains(&ext)
        });

    for path in path_iter {
        println!("{:?}", path);
    }
}
