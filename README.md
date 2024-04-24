# one_billion_row_challenge
This is a basic implementation of the one billion row challenge in Rust.

## Single threaded implementation
- Uses less than 5MB of RAM
- Runs in under 90 seconds on an 8 core AMD Ryzen 7 PRO 7840U laptop

## Multi threaded implementation
- Uses around 15GB of RAM
- Runs in under 30 seconds on an 8 core AMD Ryzen 7 PRO 7840U laptop

## Using the apache arrow datafusion library
- Uses around 80MB of RAM
- Runs in under 12 seconds on an  8 core AMD Ryzen 7 PRO 7840U laptop and under 7 seconds on a 12 core Ryzen 5900x
- Special thanks for [JosiahParry](https://github.com/JosiahParry) for the YouTube tutorial and idea

Release binaries were used for testing.

# References
- [1brc](https://github.com/gunnarmorling/1brc)
