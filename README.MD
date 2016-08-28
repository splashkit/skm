## Synopsis

SplashKit Manager or SKM is the CLI/App tool for installing and managing SplashKit, as well as creating, building, and running your very own SplashKit projects!

## Status

*Under Heavy Development*

## Use Example

* ```skm install```
* ```cd /code/projects```
* ```skm create -l pascal -n helloWorld```
* ```skm build <build args>```
* ```skm run```

## Motivation

A short description of the motivation behind the creation and maintenance of the project. This should explain **why** the project exists.

## Installation

TODO

## API Reference

### Managing SplashKit Install
* ```skm install```: Installs the latest version of SplashKit.

* ```skm update```: Updates the installed of SplashKit, if there is a newer version available.

### Managing SplashKit Projects

* ```skm init```: Initializes the current directory as a SplashKit project directory.
  * ```--language <language>```: *necessary* Specify the project language.
  * Use this command if you have an existing SplashKit project folder you would like to build.

* ```skm create```: Creates a new SplashKit project with the given language and name.
  * ```--language <language>```: *necessary* Specify the project language.
  * ```--name <projectName>```: *optional* Specify the name of the project directory. Without this the project will be created in the current directory.

* ```skm build <>```: Builds the current SplashKit project.
* ```skm run```: Runs the current SplashKit project.

## Contributors

* Jake Renzella.
https://github.com/jakerenzella/

* Alex Cummaudo

## License

Permission to use, copy, modify, and/or distribute this software for any purpose with or without fee is hereby granted, provided that the above copyright notice and this permission notice appear in all copies.
