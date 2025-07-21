# ChromaTune

A Chromatic tuner written mainly in Rust, using [redbadger/crux](https://github.com/redbadger/crux).

The iOS app UI is in Swift (in iOS/ directory), and soon there's going to be a Web UI for it.

## Build and Run

```sh
cargo build
```

Open the `iOS/ChromaTune.xcodeproj` in Xcode, and run.

## Testing the tuner via the CLI:

In the `cli/` dir, run:

```sh
cargo run detect-pitch --  ../resources/E2.wav      󱃾 qsk 21:08:33
```

We should see output similar to:

```
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.06s
     Running `/Volumes/Code/ChromaTune/target/debug/cli detect-pitch -- ../resources/E2.wav`
E2
```

Given the note played in the wav file is E2, it is important we see `E2` here.
