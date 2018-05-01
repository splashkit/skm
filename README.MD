## Synopsis

SplashKit Manager or SKM is the CLI/App tool for installing and managing SplashKit, as well as creating, building, and running your very own SplashKit projects!

## Use Example

* ```cd /code/projects```
* ```mkdir helloworld```
* ```cd helloworld```
* ```skm new c++```
* ```skm clang++ *.cpp -o HelloWorld```
* ```./HelloWorld```

## Motivation

SKM is designed to simplify the process of compiling and running programs that use the SplashKit library.

## Installation

Refer to the SplashKit [installation guide](http://www.splashkit.io/articles/installation/).

## API Reference

### Managing SplashKit Install
* ```skm install```: Installs the latest version of SplashKit.

* ```skm update```: Updates the installed of SplashKit, if there is a newer version available.

### Managing SplashKit Projects

* ```skm new <language>```: Initializes the current directory as a SplashKit project directory. Note, dotnet uses ```skm dotnet new``` - as dotnet core has its own project boostrapping process.
* ```skm <compiler> <args>```: Call the associated compiler, with the SplashKit options as well as the additional arguments.

## Contributors

* Jake Renzella.
https://github.com/jakerenzella/

* Alex Cummaudo

* Andrew Cain
https://github.com/macite/

## License

Permission to use, copy, modify, and/or distribute this software for any purpose with or without fee is hereby granted, provided that the above copyright notice and this permission notice appear in all copies.
