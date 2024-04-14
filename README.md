# Description
## konachan-yew
This is a better experience web for [konachan](https://konachan.net/), but this is just the web front, doesn't include the backend.  [konachan-api](https://github.com/lf-wxp/konachan-api) is the backend server supplying image data.

# Screenshot

![screenshot](./screenshot.png)

# Development
first install [rust](https://www.rust-lang.org/) development chain.

then

```bash
cargo install trunk
trunk server --features fake
``` 

# Reference

[konachan-api](https://github.com/lf-wxp/konachan-api) the konachan image data server

[konachan-tauri](https://github.com/lf-wxp/konachan-tauri) the desktop version supported by tauri framework
