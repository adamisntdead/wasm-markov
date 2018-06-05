# Markov WebAssembly

> Markov Chains Implemented in Rust, Powered By WebAssembly 🔥

## Usage

First you are gonna have to get Rust at [rust-lang.org](https://www.rust-lang.org/).
Then, you will need to have [nodejs](https://nodejs.org/) installed.

For the WebAssembly side of things, you are going to have to get to Rust nightly,
and install the WebAssembly compiler.

```bash
rustup default nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
```

Then you need to install the dependencies and compile the WebAssembly

```
npm install
npm run build-debug
npm run serve
```

## License

MIT