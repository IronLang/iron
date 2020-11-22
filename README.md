<img src=".github/assets/logo.svg" alt="Iron logo" height="128px">

# The Iron Programming Language

[![Rust](https://github.com/ironlang/iron/workflows/Rust/badge.svg)](https://github.com/ironlang/iron/actions?query=workflow%3ARust)
[![Discord](https://img.shields.io/discord/745404935193231372?color=blueviolet&label=Discord&logo=discord&logoColor=white)](https://discord.gg/REZTE6T)

> **Iron is a programming language for building secure and scalable systems.** This repository contains the source code for the Iron compiler, which is currently implemented as a collection of Rust crates.

Iron is not production-ready by any means, but is actively being built in the open. I welcome anyone interested in shaping its future.

## Hello, world!

Iron takes inspiration from Rust, as well as other modern languages including Go, Swift, and TypeScript. Below is an example "hello world" program.

```iron
import "io"

// Represents a color.
type RGB (U8, U8, U8)

// Represents a person.
type Person {
	name          String
	favoriteColor RGB?
}

implement Person {
	// Returns a string introducing the `Person`.
	introduce(self Self) String {
		"Hi, my name is ${self.name}!"
	}
}

function main() {
	let name = "Sam"
	let favoriteColor = (0, 122, 255)
	let author = Person{name, favoriteColor}

	io.print(line: author.introduce())
}
```

Iron aims to maintain a small conceptual surface area. This is reinforced with other language decisions that get out of your way and guide you toward code that "just works."
