# rust-chess
Multiplayer chess game in rust

## building

requires the nightly rust compiler, which can be installed (on windows) with

```
rustup toolchain install nightly-msvc
```

set cargo to use the git CLI instead of libgit as it's a lot more reliable
```
cargo --config "net.git-fetch-with-cli = true"
```


and running build command with +nightly option
```
cargo +nightly build
```

or just set it to run by default for this project
```
rustup override set nightly
```

install visual studio 2019 community through website, not chocolately

