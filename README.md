Read file/stdin and find all staring with '<pjd>' and ending with '</pjd>'

**USAGE**:

    collect_pjds [OPTIONS] [INPUT]

**ARGS**:

    <INPUT>    Path to file to read data from

**OPTIONS**:
    
    -h, --html-decode        Shall we decode HTML entities before search
        --help               Print help information
    -o, --output <OUTPUT>    Path to directory where results will be stored (will be created if does
                             not exist) [default: ./output]
    -s, --stdin              Read data from standard input
    -V, --version            Print version information
