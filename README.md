<picture>
    <source srcset="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_Solid_White.svg" media="(prefers-color-scheme: dark)">
    <img src="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_RGB.svg" alt="Leptos Logo">
</picture>

# Full stack rust with leptos

Talk for the RustBCN meetup group. 

This is a self-contained slide deck to show what leptos can be capable of. 

## Running your project

```bash
cargo leptos watch
```

## Executing a Server on a Remote Machine Without the Toolchain
After running a `cargo leptos build --release` the minimum files needed are:

1. The server binary located in `target/server/release`
2. The `site` directory and all files within located in `target/site`

Copy these files to your remote server. The directory structure should be:
```text
rustbcn_leptos
site/
```
Set the following environment variables (updating for your project as needed):
```sh
export LEPTOS_OUTPUT_NAME="rustbcn_leptos"
export LEPTOS_SITE_ROOT="site"
export LEPTOS_SITE_PKG_DIR="pkg"
export LEPTOS_SITE_ADDR="127.0.0.1:3000"
export LEPTOS_RELOAD_PORT="3001"
```
Finally, run the server binary.

## Licensing

Original template and logo by Leptos project. Some code is from leptos_ws. 

This code cannot be used to train AI. Can't be used as is for commercial purposes. 
You must use for personal purposes or make any derivations available to anyone. 
