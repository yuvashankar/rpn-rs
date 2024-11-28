# Reverse RPN Calculator
A simple [Reverse Polish Notation](https://en.wikipedia.org/wiki/Reverse_Polish_notation) calculator built in Rust. 

This was an example problem from the [Learn you a Haskell for great good](https://www.learnyouahaskell.com/functionally-solving-problems).
In the book the author walks through the solution in Haskell. 

I wanted to see if I could apply the same principals with Rust. Since functional programming is essentially computation without state mutation, you'll notice that there are no mutable variables within the code block. I had to cheat a little bit and create a mutable vector witin a closure, it's bounded I guess.

## Nix-ey stuff
This was also an experiment in learning how to create a Rust Flake with NixOS. 
Flake was initialized from the rust template.

```bash
nix flake init --template templates#rust
```
