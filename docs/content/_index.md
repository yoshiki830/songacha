---
title: "songacha"
---

# songacha

songacha is a command line song gacha tool using Perfume songs as sample data.

It randomly draws songs from a local CSV file and records pull history in a local save file.

## What can it do?

- Draw one or more songs randomly
- Save pull history
- Show collected songs
- Show missing songs
- Show collection progress
- Show frequently drawn songs ranking

## Example

Draw three songs:

    songacha pull --count 3

Draw songs with a fixed random seed:

    songacha pull --count 3 --seed 42

Show collection progress:

    songacha collection

Show missing songs:

    songacha collection --missing

Show ranking:

    songacha ranking

## Sample data

The default sample data contains 52 songs from Perfume The Best P Cubed.

The CSV file is located at:

    data/perfume_p_cubed.csv

## Notes

songacha does not play music, display lyrics, or use artist images.

It only handles local song metadata such as song titles, artist names, album names, and disc numbers.

This project is unofficial and is not affiliated with Perfume or any related organizations.
