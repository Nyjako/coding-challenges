# Coding Challenges

This repository contains various coding challenges I am working on. Each challenge is stored in its own subdirectory, and inside you'll find a `REFERENCES.md` file with additional details about the references and inspirations for each specific challenge.

## Challenges by language

- **Rust:**
    - [01-falling-sand](./01-falling-sand)
    - [02-reaction-diffusion-algorithm](./02-reaction-diffusion-algorithm)

- **C++:**
    - [03-classic-cube](./03-classic-cube)

## Requirements

To build and run the challenges, you will need the following:

- **Rust Challenges:** 
    Rust projects are built using the [Nannou](https://nannou.cc/) creative coding framework.  
    Please refer to the [Platform-specific Setup](https://guide.nannou.cc/getting_started/platform-specific_setup) guide for installation instructions.
- **C++ Challenges:**
    C++ challenges are written using C++17 and the [Cinder](https://libcinder.org/) framework.  
    Refer to the [Platform Setup Guides](https://www.libcinder.org/docs/guides/linux-notes/) for setup information.

## Building Instructions

### Rust
To build the Rust challenges:

```bash
cargo build --release
```
> [!IMPORTANT]  
> Build in release mode for better performance.

### C++
To build the C++ projects:

```bash
cmake -S . -B build
cmake --build build
```