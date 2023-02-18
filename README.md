<h1 align="center">
  📖 Poketex
</h1>

<p align="center">
  Simple Pokedex based on TUI(Terminal User Interface)
</p>

![demo](doc/demo.png)

data generated from [ckaznable/poke-data-crawler](https://github.com/ckaznable/poke-data-cralwer)

## Installation

Download the last version binary depending on your configuration here: [Release Page](https://github.com/ckaznable/poketex/releases/tag/v1.0.0)

Then you just need to enter this command in your terminal:

```shell
tar -xf <downloaded_archive> poketext && sudo mv poketext /usr/local/bin
```

## Usage

```shell
Poketex, Simple Pokedex based on TUI(Terminal User Interface)

Usage: poketex [OPTIONS]

Options:
  -l, --locale <LOCALE>  locales [zh, ja, en] [default: en]
  -h, --help             Print help
  -V, --version          Print version
```

## Feature

1. All Pokemon (until Pokemon SV)
2. All Region Form (until Pokemon SV)
3. Searchable
4. include english, chinese, japanese info data

## Todo

- [x] show pokemon iv and type
- [x] pokemon list searchable
- [x] add pokemon ability rows
- [x] add region form
  - [x] Alola
  - [x] Galar
  - [x] Hisuian
  - [x] Paldea
- [x] add cli variable
  - [x] --locale [en, ja, zh]
- [ ] feature tab
  - [ ] damage calculator
