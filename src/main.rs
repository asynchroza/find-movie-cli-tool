use clap::Parser;
use walkdir;
use std::process::Command;
use dirs;

#[derive(Parser, Debug)]
struct Cli {
    /// If you want to add a custom path which is outside your default Videos directory
    #[clap(short = 'p', long = "path", default_value_t=dirs::video_dir().to_owned().unwrap().to_str().unwrap().to_string())]
    path: String,
    
    /// Name of the movie you want to find and watch
    #[clap(short = 'm', long = "movie")]
    movie: String,
}

fn search_for_movie(args: &Cli){

    println!("Hold on! We are searching for the movie you requested!");

    let path = walk_files(&args.path, &args.movie);

    if path != "" {open_vlc(&path)}
    else {
        println!("Movie was not found, exiting!");
    }
    
}

fn check_filename(filename: &String, movie_name: &String) -> bool {

    let spaced_movie_name = movie_name.split_whitespace();
    for word in spaced_movie_name{

        if !filename.contains(".mkv") {
            return false;
        }

        if !filename.contains(word) {
            return false;
        }
    }
    
    true
}

fn open_vlc(file_path: &String) {
    if !cfg!(target_os = "windows"){
                    println!("Movie file path is: {}", file_path);
                    Command::new("vlc")
                        .arg(file_path)
                        .output()
                        .expect("failed to execute process");
                }
}

fn walk_files(path: &String, movie_name: &String) -> String {
    for file in walkdir::WalkDir::new(path).into_iter().filter_map(|e| e.ok()){
        if file.metadata().unwrap().is_file() {
            if check_filename(&file.path().file_name().unwrap().to_str().unwrap().to_owned().to_lowercase(), &movie_name.to_lowercase()) {
                return file.path().to_str().unwrap().to_owned()
            }
        }
    }
    
    String::from("")
}

fn main() {
    let args = Cli::parse();
    search_for_movie(&args);
}
