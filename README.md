# A tool for querying Valve MvM servers

```
USAGE:
    mvmtool <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    find             Finds MvM matches
    help             Prints this message or the help of the given subcommand(s)
    list-missions    Lists known missions
    set-key          Sets the Steam web API key
```
```
Finds MvM matches

USAGE:
    mvmtool find [FLAGS] [OPTIONS]

FLAGS:
    -h, --help            Prints help information
        --skip-players    Skip player queries
    -V, --version         Prints version information

OPTIONS:
    -m, --mission <mission>    Mission to search
    -t, --tour <tour>          Tour to search
```

## License

[Public Domain](/LICENSE)
