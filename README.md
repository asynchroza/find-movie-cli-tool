# CLI tool for finding a local movie and playing it (on VLC)
### Works only on Linux/GNU and UNIX platforms
Installing the program (make sure you have rust installed - cargo):
```bash
./install.sh  # run the script in the root directory of the project
```

Running the program:
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

