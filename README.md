# EngLit
## About
This is a high level programming language compiler that uses English-based syntax. The focus is to reduce the amount of non-alphanumeric glyphs when interfacing with a program.

## Why
Because I said so.

## Inspiration
When I first started learning how to program, I found the logic aspects of telling a computer what to do really fascinating. From there, I always wondered what it would be like to "talk" to a computer exactly like a human would.

## What EngLit is NOT
- EngLit is **NOT** meant to be a full replacement for programming languages.
- EngLit is **NOT** meant to be a perfect English language that interfaces with a computer. If you want that, see [here](https://chat.openai.com/). Building a perfect English compiler would require an immense amount of time and effort than I already have.
- As such, because there are multiple ways to say the same thing in English (*every language really is an art form*) I have chosen a specific syntax that I thought was elegant.

# Getting Started
## Prerequisites
This program uses GNU Linker and NASM to compile the assembly code. It also uses Rust to compile the program.
Currently, this program only works on Linux. You may build an object file on Windows, but you will need to use a Linux machine to link the object file.

## Installation
1. Clone the repository.
2. Make sure you have [NASM](https://www.nasm.us/) installed.
3. Make sure you have [Rust](https://www.rust-lang.org/) installed.

## Running the program
1. Compile the program by running `rustc ./src/main.rs`.
2. Run `./main <filename>.elit` to compile the program.
3. Run `./<filename>` to run the program.