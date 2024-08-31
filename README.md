## FILE UPLOAD WebApp Using RUST 
A demo application if how you can store images and videos on the cloud.
Whether you are a fan of Rust or are a developer interested in new frameworks, Yew is a strong option that allows for fast, reliable, and maintainable web applications.
<br>
## What is Yew?
Yew is an incredibly versatile tool that allows web developers to craft their front-end projects on the web using Rust and WASM. Compiling Rust into WASM enables the design of excellent-speed web apps resembling desktop applications involving episode processing rates.

## Setting Up the Environment for a Yew Project
 - Launch a terminal and execute the installation command for Rust:
 
````
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
````
- After installation, ensure that Rust and cargo are available in your terminal by checking their versions:

```
rustc --version
cargo --version
```
- The next action you have at hand is to install wasm-pack, a tool designed to assist in building Rust-generated WebAssembly packages. It streamlines the compilation of Rust program files into WebAssembly modules and makes them ready for use with websites and applications developed in Internet-based platforms. By executing this command, you can get wasm-pack installed:

```
cargo install wasm-pack
```
- Add trunk, a build tool for various Rust projects that builds and bundles Yew applications.
```
cargo install trunk
```


## Run this project
- Clone this repository
```
git clone https:://github.com/Prashidha0O1/yew-rs.git
```
- build this
```
cargo build
```
- run this project
```
trunk serve --open
```

