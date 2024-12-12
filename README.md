# Reverse Polish Notation Calculator

A simple [Reverse Polish Notation](https://en.wikipedia.org/wiki/Reverse_Polish_notation) calculator built in Rust. 

Running the line will result in 
```bash
$ revrse rpn "10 4.5 +"
$ 14.5
```

This example, only implements addition and multiplication, if you'd like I've filled in enough hints to allow you to implement the subtraction and division operations. 

## Development

There are a few ways to build this package. 
1. Using the rust toolchain on your current operating system
2. Using the provided nix flake. (hard mode)


### Develop with Rust
Install the [rust toolchain](https://www.rust-lang.org/tools/install), and ensure that you are able to access the cargo CLI tool.
```bash
cargo --version
```

Once cargo is installed, build the package
```rust
cargo build
```

and run with 
```bash
cargo run -- "10 4 +"
```

Run the unit tests with 

```bash
cargo test
```

### Develop with Nix Flakes


1. Install the [nix package manager](https://nixos.org/download/), and enable [nix flakes](https://nixos.wiki/wiki/flakes). 
2. Install the [direnv](https://direnv.net/docs/installation.html) package.
3. Open the directory and run 
```bash
nix develop
```
Which should download the necessary resources, allowing you to run 
```bash
cargo -version
```
See the [rust](#develop-with-rust) section for instructions building the package.

The entire package can be built with nix flakes using 
```bash
nix build
```
Which puts the compiled binary in the `result` directory. 


## Motivation
This was an example problem from the [Learn you a Haskell for Great Good](https://www.learnyouahaskell.com/functionally-solving-problems).
In the book the author walks through the solution in Haskell. 

I wanted to see if I could apply the same principals with Rust. Since functional programming is essentially computation without state mutation, you'll notice that there are no mutable variables within the code block. I had to cheat a little bit and create a mutable vector within a closure, it's bounded I guess. I think there is a better way i.e. using `reduce`. This is an exercise left to the reader.

I also wanted to use this as an oppurtuinity to learn how to build a nix flake for rust. 

## Footnotes
## Initializing a nix repo for Rust
The documentation on nix and nix flakes is somewhat fractured, and depending on when you are reading this, the information may be out of date or not the prescribed method of doing it. These are the steps that I had to take to be able to develop rust with a nix flake.

Using VSCode/Vscodium's [rust-analyzer](https://rust-analyzer.github.io/) extension is not possible by just `cd`-ing into the directory. These are the steps that I took to be able to develop with rust-analyzer.

    
```bash
nix flake init --template templates#rust
```

### 1. Install `direnv`
The nix flake init templates create a `.envrc` file that contains the modifications that need to be run to enable rust analyzer to properly run.

### 2. Configure vscode in home-manager with the `direnv` extension
See [this](https://github.com/yuvashankar/nix-config/blob/9e19c7954b616ce83e37663efc2e445054ecd728/home/features/productivity/common/vscodium.nix#L1) commit for the config that worked.

## Open questions
* Adding extensions with VSCodium from within the flake adds it globally, not ideal.
* Have been unable to setup a debugger from within the nix flake. [May not be possible right now?](https://a.ndr.ooo/Nix--and--Rust--and--VSCode#addendum-debugging)