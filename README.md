<img src=".github/assets/logo.svg" alt="Iron logo" height="64px">

# The Iron Programming Language

[![Rust](https://github.com/ironlang/iron/workflows/Rust/badge.svg)](https://github.com/ironlang/iron/actions?query=workflow%3ARust)
[![Discord](https://img.shields.io/discord/745404935193231372?color=blueviolet&label=Discord&logo=discord&logoColor=white)](https://discord.gg/REZTE6T)

> **Iron is a programming language for building secure and scalable systems.** This repository contains the source code for the Iron compiler, which is currently implemented as a collection of Rust crates.

Iron is not production-ready by any means, but is actively being built in the open. I welcome anyone interested in shaping its future.

Iron takes inspiration from Rust, as well as other modern languages including Go, Swift, and TypeScript. Below is an example "hello world" program.

```iron
import io

// Represents a person.
type Person = {
	// The name of the person.
	name: String
}

// Greets someone based on their name.
function greet(person: Person): String => "Hello, {person.name}!"

function main {
	let person = Person{ name: "J. Doe" }
	io.print(greet(person)) // prints "Hello, J. Doe!"
}
```

The language strives to maintain a minimal feature surface area, and will provide opionated libraries for building production-quality software at scale.
