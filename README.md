# CLI tool for finding a movie and playing it (in VLC)
It's expected that you store your local movies under the `XDG_VIDEOS_DIR` (e.g. ~/Videos on Linux).  
Storing a movie in a child directory is not a problem due to the fact that the tool utilizes recursive searching.
### Works only on Linux/GNU and UNIX platforms
Installing the program (make sure you have rust installed - cargo):
```bash
./install.sh  # run the script in the root directory of the project
```

Running the program (make sure you have vlc installed or modify the command to suit your needs #Line50:main.rs):
```bash
find-movie-cli-tool -m "Your Movie Here" # do not forget "" when passing the movie as an argument
```

Different flags:
```bash
OPTIONS:
    -h, --help             Print help information
    -m, --movie <MOVIE>    Name of the movie you want to find and watch
    -p, --path <PATH>      If you want to add a custom path which is outside your default Videos
                           directory [default: /home/<user>/Videos]
```

