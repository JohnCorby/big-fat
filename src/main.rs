use walkdir::WalkDir;

/// hardcoded cuz lmao
const PATH: &str = r"D:\OneDrive - Lake Washington School District\Everything Else\gay\sound is gay\vgm server (ie parker)";

fn main() {
    // read all paths recursively, ignoring errors
    let path_iter = WalkDir::new(PATH)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.into_path())
        .filter(|path| path.is_file());

    for path in path_iter {
        println!("{:?}", path);
    }
}
