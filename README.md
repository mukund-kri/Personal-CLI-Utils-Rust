# Personal-CLI-Utils-Rust

In this project I am trying to migrate all my personal CLI utilities, from mostly
Python to Rust. 

It's vanity project, the python version works great, I just want to do it in Rust.

## Utilities

### `new_note`
A utility to create notes on a particular topic. I mostly use it with
`100 days of programming X` challenge.

**Usage**
```bash
new_note <topic>
```

where `<topic>` is the topic of the note in snake_case.

## Installation

In the cargo root directory, run the following command:

```bash
cargo install --path .
```