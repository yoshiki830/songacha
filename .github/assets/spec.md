# songacha Specification

## Purpose

songacha is a command line tool that provides a song gacha experience.
The tool randomly draws songs from a local song list and manages simple collection data.

This project uses Perfume songs as sample data.
The goal is to create a small but complete CLI application while following the software development process.

## Target Users

The target users are people who want to choose a song randomly from a predefined song list.
For example, a user can use this tool when they do not know which Perfume song to listen to.

## Input

songacha uses local song metadata as input.

The song data is stored in a CSV file.
Each record contains the following fields:

- id
- title
- artist
- album
- disc

Example:

    id,title,artist,album,disc
    1,Challenger,Perfume,Perfume The Best P Cubed,1
    2,リニアモーターガール,Perfume,Perfume The Best P Cubed,1

## Output

The tool outputs text to the terminal.

The main outputs are:

- gacha result
- collected songs
- missing songs
- collection progress
- frequently drawn songs ranking

## Commands

### pull

Draw one or more songs from the song list.

Example:

    songacha pull
    songacha pull --count 10

### collection

Show collected songs and collection progress.

Example:

    songacha collection
    songacha collection --missing

### ranking

Show frequently drawn songs.

Example:

    songacha ranking

### reset

Reset local save data.

Example:

    songacha reset

## Non-goals

songacha does not play music.
songacha does not display lyrics.
songacha does not use artist images.
songacha does not access external music APIs.

The tool only handles local song metadata.
