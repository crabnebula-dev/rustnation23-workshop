# Hands-on with Tauri & Yew

This is the code accompanying the Rust Nation 2023 hands-on workshop. 

## Prerequisites

1. Setup Tauri prerequisites for your OS (especially on Linux): 
   https://tauri.app/v1/guides/getting-started/prerequisites/

2. Install the wasm32 Rust target

    ```
    rustup target install wasm32-unknown-unknown
    ```

3. Install the Tauri CLI

    ```
    cargo install tauri-cli
    ```

4. Install Trunk

    ```
    cargo install trunk
    ```

5. Clone this repo

    ```
    git clone https://github.com/crabnebula-dev/rustnation23-workshop
    ```

## Structure

This repo contains a handful or numbered snapshots (1 through 4) in the `./checkpoints` subfolder that correspond to the numbered stages on the slides. You can check out any of these to time-travel or skip ahead. 

There is also a `complete` checkpoint which is my take on the RSS-reader app shown at the beginning of the talk.

