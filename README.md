<img src="https://www.history.com/.image/t_share/MTU3ODc4Njg0NTgzNTM1OTQ1/image-placeholder-title.jpg" width="100" height="100">


# rlox
Feature-complete Lox interpreter from Bob Nystrom's [Crafting Interpreters](https://craftinginterpreters.com/).

## Overview
rlox is a Lox interpreter implemented in Rust. The original architecture of the interpreter is tree-walk based, and heavily influenced by OOP. My implementation in Rust forgos the book's Visitor's Pattern design and opts for a more Rust-like solution with enums and pattern matching. The main components of the interpreter consist of the lexer, parser, resolver and interpreter.

## Contents
| directory/file       | description                                                                                                    |
| -------------------- | -------------------------------------------------------------------------------------------------------------- |
| src/                 | Directory with Lox interpreter implementation                                                                  |
| src/environment.rs   | Holds a given scope's values for the interpreter                                                               |
| src/interpreter.rs   | Executes statements                                                                                            |
| src/main.rs          | Runs Lox code from a file or in a REPL on the command line                                                     |
| src/parser_.rs       | Turns tokens from the scanner into a syntax tree                                                               |
| src/resolver.rs      | Resolves variable scopes using the syntax tree from the parser                                                 |
| src/scanner.rs       | Turns raw Lox source code into tokens                                                                          |
| src/token.rs         | Types for tokens and literals                                                                                  |

## Resources
- [Compilers, Stanford's course taught by Alex Aiken](https://online.stanford.edu/courses/soe-ycscs1-compilers)
- [Compilers: Principles, Techniques, and Tools](https://en.wikipedia.org/wiki/Compilers:_Principles,_Techniques,_and_Tools)
