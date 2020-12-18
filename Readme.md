# 2048 WebAssembly

<p align="center">
  <img src="https://github.com/dev-family/wasm-2048/blob/master/images/result.png">
</p>

<p align="center">
  <a href="https://2048.dev.family">Live Demo</a>
</p>

[The famous 2048 game](https://github.com/gabrielecirulli/2048) implemented with Rust ([Yew](https://yew.rs/)) and compiled to WASM.

# Running

The simplest way to run is via docker:

```
docker build -t wasm-2048 .
docker run -it --rm -p 8080:8080 wasm-2048
```

Then open http://127.0.0.1:8080 and that's it!
