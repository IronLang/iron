# Contributing to Iron

**First and foremost, thank you for your interest!** If you are reading this then you are probably interested in programming language design, and perhaps the Iron language specifically. Either way, I think that is pretty cool.

At this point, Iron is just a personal side project I am building to learn more about compilers. That said, there are a lot of different ways you can get involved if you're interested in learning together.

## Project Areas

### The Iron compiler

The Iron compiler is broken into multiple different Rust crates. At this point, it is specifically an LLVM front-end. If you have experience with (or are interested in learning more about) LLVM, I'd start by digging into the `iron_codegen_llvm` crate. There are also `iron_lexer` and `iron_parser` crates which operate on Iron programs at a higher level.

### Core and standard packages

In Iron, the `core` package will be implicitly baked into projects, though it will be able to be excluded. The `standard` package extends this; my mission is for Iron to have a comprehensive, tried-and-true standard collection of modules similar to Go's stdlib.

### Forge, the Iron project toolchain

Forge is meant to be a comprehensive CLI for initializing and managing Iron software projects. Right now it is mostly a testbed for the different compiler crates, but I do need to spend some time figuring out its design.

### The Iron website

The Iron website will serve as a "book" for the language, but will also be the home of the package index.

### Community building

Even if none of the above areas interest you, you are still welcome to hang out in our Discord. Share your thoughts on proposals, syntax, and whatever else comes to mind. As long as you follow our code of conduct, anyone and everyone is welcome.
