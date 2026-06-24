# songacha

[![build](https://github.com/yoshiki830/songacha/actions/workflows/build.yaml/badge.svg)](https://github.com/yoshiki830/songacha/actions/workflows/build.yaml)

A command line song gacha tool using Perfume songs as sample data.

## Overview

songacha is a CLI tool that randomly draws songs from a local song list.

This project uses Perfume songs as sample data and provides gacha-like features such as pull results, collection progress, missing songs, and frequently drawn song rankings.

The goal of this project is to build a small but complete CLI application while learning the software development process, including documentation, testing, CI/CD, and release management.

This tool does not play music, display lyrics, or use artist images.
It only handles local song metadata such as song titles, artist names, album names, and disc numbers.

## Features

- Draw a random song from a song list
- Draw multiple songs at once
- Save pull history locally
- Show collected songs
- Show missing songs
- Show collection progress
- Show frequently drawn songs ranking

## Usage

Run the following command to draw one random song.

    songacha pull

Draw ten songs.

    songacha pull --count 10

Show collected songs and collection progress.

    songacha collection

Show songs that have not been collected yet.

    songacha collection --missing

Show frequently drawn songs.

    songacha ranking

Reset local save data.

    songacha reset

## Installation

This project is written in Rust.

    cargo install --path .

After installation, run:

    songacha --help

## Data Format

Song data is managed as a local CSV file.

Example:

    id,title,artist,album,disc
    1,Challenger,Perfume,Perfume The Best P Cubed,1
    2,リニアモーターガール,Perfume,Perfume The Best P Cubed,1
    3,コンピューターシティ,Perfume,Perfume The Best P Cubed,1

## About

songacha was created as a coursework project for empirical software engineering.

The name songacha comes from "song" and "gacha".
It means a small CLI tool that randomly draws songs like a gacha.

This project is unofficial and is not affiliated with Perfume or any related organizations.


## Website

The project website is available at:

    https://yoshiki830.github.io/songacha/

## License

This project is licensed under the MIT License.

## Current Implementation

The current version supports the following features:

- Load song metadata from a local CSV file
- Draw one or more songs randomly
- Use a random seed for reproducible results
- Save pull history to a local JSON file
- Show collected songs
- Show missing songs
- Show collection progress
- Show frequently drawn songs ranking
- Reset local save data

## Examples

Draw three songs with a fixed seed:

    songacha pull --count 3 --seed 42

Show collected songs:

    songacha collection

Show missing songs:

    songacha collection --missing

Show frequently drawn songs:

    songacha ranking

Reset save data:

    songacha reset

Use a custom song data file:

    songacha --data-file data/perfume_p_cubed.csv pull

Use a custom save file:

    songacha --save-file save.json pull

    songacha --save-file save.json pull

## Sample Data

The default sample data contains 52 songs from Perfume The Best P Cubed.

The CSV file is located at:

    data/perfume_p_cubed.csv
