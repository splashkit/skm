# SplashKit Manager (SKM)

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

**SplashKit Manager** (SKM) is a command-line interface (CLI) and app tool designed for installing, managing, and building projects with the SplashKit SDK. With SKM, you can easily create, compile, and run SplashKit projects, making it a streamlined tool for beginners and experienced developers alike.

## Usage Example

To start a new SplashKit project in C++, follow these steps:

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

## Key Commands

- **`skm install`**: Installs the latest version of SplashKit.
- **`skm update`**: Checks for and installs the latest version of SplashKit if available.
- **`skm help`**: Displays a list of available commands and their usage.
- **`skm uninstall`**: Uninstalls SplashKit Manager.

## Full Command List

### Project Management and Setup

- **`skm new <language>`**: Initializes a new SplashKit project in the current directory for the specified language (e.g., `c++`, `python`).  
  - *Note*: Use `skm dotnet new` for .NET projects.

- **`skm resources`**: Creates the resources folders used by SplashKit for assets like images, sounds, etc.

### Compilers and Environment-Specific Commands

- **`skm clang++ <args>`**: Runs the clang++ (or g++) compiler with the specified arguments and SplashKit settings.  
  - Example: `skm clang++ program.cpp -o HelloWorld` then `./HelloWorld`
  
- **`skm g++ <args>`**: Runs the g++ compiler with SplashKit-specific options and any additional arguments.

- **`skm fpc <args>`**: Runs the Free Pascal compiler with SplashKit-specific options.

- **`python3 <args>`**: Runs Python with SplashKit settings (SplashKit is installed globally for Python, so `skm` is not needed).  
  - Example: `python3 program.py`

- **`skm dotnet <args>`**: Runs the .NET command line tool with SplashKit-specific settings.

### Platform-Specific Commands

- **`skm linux`**: Installs SplashKit for Linux distributions.
  - Example: `skm linux install`

- **`skm macos`**: Installs SplashKit for macOS.
  - Example: `skm macos install`

### Utility Commands

- **`skm global`**: Sets up SplashKit for global installation, enabling it to work across multiple projects.

- **`skm revert`**: Downgrades to an earlier version of SplashKit.

- **`skm fix`**: Corrects path issues in SplashKit project files, resolving common setup issues.

- **`skm help [command]`**: Displays usage information for `skm` commands.
  - Examples: `skm help`, `skm help clang++` or `skm help python3`

## Contributor Guide

We welcome contributions to SplashKit Manager! Whether you're reporting an issue, suggesting new features, or submitting code, all contributions help improve SKM. To contribute, please open a pull request or start a discussion in the repository.

## License

This software is free to use, copy, modify, and distribute for any purpose, with or without fee. Please ensure the copyright notice and permission are included in all copies.
