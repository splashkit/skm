<p align="left">
    <img width="150px" src="https://github.com/thoth-tech/.github/blob/main/images/splashkit.png"/>
</p>

### [Splashkit](https://github.com/splashkit/skm) Stats

[![GitHub contributors](https://img.shields.io/github/contributors/splashkit/skm?label=Contributors&color=F5A623)](https://github.com/splashkit/skm/graphs/contributors)
[![GitHub issues](https://img.shields.io/github/issues/splashkit/skm?label=Issues&color=F5A623)](https://github.com/splashkit/skm/issues)
[![GitHub pull requests](https://img.shields.io/github/issues-pr/splashkit/skm?label=Pull%20Requests&color=F5A623)](https://github.com/splashkit/skm/pulls)
[![Forks](https://img.shields.io/github/forks/splashkit/skm?label=Forks&color=F5A623)](https://github.com/splashkit/skm/network/members)
[![Stars](https://img.shields.io/github/stars/splashkit/skm?label=Stars&color=F5A623)](https://github.com/splashkit/skm/stargazers)

### [Thoth Tech](https://github.com/thoth-tech/skm) Stats

[![GitHub contributors](https://img.shields.io/github/contributors/thoth-tech/skm?label=Contributors&color=F5A623)](https://github.com/thoth-tech/skm/graphs/contributors)
[![GitHub issues](https://img.shields.io/github/issues/thoth-tech/skm?label=Issues&color=F5A623)](https://github.com/thoth-tech/skm/issues)
[![GitHub pull requests](https://img.shields.io/github/issues-pr/thoth-tech/skm?label=Pull%20Requests&color=F5A623)](https://github.com/thoth-tech/skm/pulls)
[![Forks](https://img.shields.io/github/forks/thoth-tech/skm?label=Forks&color=F5A623)](https://github.com/thoth-tech/skm/network/members)
[![Stars](https://img.shields.io/github/stars/thoth-tech/skm?label=Stars&color=F5A623)](https://github.com/thoth-tech/skm/stargazers)


# SplashKit Manager (SKM)

**SplashKit Manager** (SKM) is a command-line interface (CLI) and app tool designed for installing, managing, and building projects with the SplashKit SDK. With SKM, you can easily create, compile, and run SplashKit projects, making it a streamlined tool for beginners and experienced developers alike.

## Usage Example

To get started with a new SplashKit project, follow these steps:

```bash
# Navigate to your projects directory
cd /code/projects

# Create and navigate to a new project folder
mkdir helloworld
cd helloworld

# Initialize a new SplashKit C++ project
skm new c++

# Compile your project using SKM
skm clang++ *.cpp -o HelloWorld

# Run the compiled program
./HelloWorld
```

## Motivation

SKM is designed to simplify the process of creating, compiling, and running programs that use the SplashKit library, eliminating the need for manual configuration and setup. It makes SplashKit accessible for developers of all skill levels by automating common tasks.

## Installation

For installation instructions, refer to the official SplashKit [Installation Guide](https://splashkit.io/installation/).

## API Reference

### Managing SplashKit Installation

- **`skm install`**: Installs the latest version of SplashKit, making it available for project use.
- **`skm update`**: Checks for updates and installs the latest version of SplashKit if a newer version is available.

### Managing SplashKit Projects

- **`skm new <language>`**: Initializes the current directory as a new SplashKit project for the specified language (e.g., `c++`, `python`).
    - **Note**: For `.NET` projects, use `skm dotnet new`, as `.NET Core` has its own project initialization process.
- **`skm <compiler> <args>`**: Compiles the project by calling the associated compiler with SplashKit-specific options and any additional arguments provided.

## Contributors

- [Jake Renzella](https://github.com/jakerenzella)
- Alex Cummaudo
- [Andrew Cain](https://github.com/macite/)

## License

Permission is hereby granted, free of charge, to any person obtaining a copy of this software to use, copy, modify, and/or distribute it for any purpose, with or without fee, provided that the above copyright notice and this permission notice appear in all copies.

## Contributing

We welcome contributions to help improve SplashKit Manager (SKM)! Whether you want to report an issue, suggest a new feature, or submit code changes, all contributions are appreciated. Feel free to open a pull request or start a discussion in the repository – together, we can make SKM better for everyone!
