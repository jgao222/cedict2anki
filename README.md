# cedict2anki

Originally this was supposed to be dictionary reading from CEDICT, but all I was
going to use it for was to get definitions for Anki, so I just did that instead.

## Build
Prequisites: have the Rust toolchain.

It's possible to run it directly with cargo, like `cargo run -- [input_file]` since
performance doesn't matter that much.

Otherwise, `cargo build --release`, then run `./target/release/ch-dict(.exe) [input_file]`.

## Usage
Takes one command line argument - path of an input file which is just a list of
Chinese characters separated by newlines. Currently only supports Simplified character lookup.

It outputs to the standard output, pipe into a file if desired, like
`cargo run -- [input_file] > [output_file]`.

Example input file - `input.txt`
```
时辰
祁南
驿道
嗡嗡
踏
啧
惨
盗
镖局
八成
```

Example corresponding output
```
时辰|"shíchen
time / one of the 12 two-hour periods of the day"
嗡嗡|"wēngwēng
buzz; drone; hum"
踏|"tà
to tread / to stamp / to step on / to press a pedal / to investigate on the spot

tā
see 踏實|踏实[ta1 shi5]"
啧|"zé
(interj. of admiration or of disgust) / to click one's tongue / to attempt to (find an opportunity to) speak"
惨|"cǎn
miserable / wretched / cruel / inhuman / disastrous / tragic / dim / gloomy"
盗|"dào
to steal / to rob / to plunder / thief / bandit / robber"
八成|"bāchéng
eighty percent / most probably / most likely"

### Lookup complete: 3 words were not found
# Could not find definition for 祁南
# Could not find definition for 驿道
# Could not find definition for 镖局
```
Note that the output looks weird since it's just how I have it set up to use
a Basic note (front+back) and newlines to separate what should really be fields
for pinyin and definitions.