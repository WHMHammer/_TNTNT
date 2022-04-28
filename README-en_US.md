# TNTNT 

`TNTNT` (**T**NTNT is **N**ot **T**aiko **n**o **T**atsujin, pronounced as "tee-en-tee-en-tee") aims to parse `.tja` files and:
- visualize per-course charts
- provide an interactive graphical interface for the users to play the charts

`TNTNT` is still in an early stage. I will not accept any external code contribution until I release the first desktop application (after finishing[stage 2](#stage-2)).

[中文（中国）](README.md)

## Table of Contents

- [Motivation](#motivation)
- [Build](#build)
- [Road Map](#road-map)
    - [Stage 1: Visualizing](#stage-1) **(developing)**
    - [Stage 0: Parsing (v0.x)](#stage-0) **(finished)**
    - [Stage 2: Desktop Application](#stage-2) (planned)
    - [Stage 3: Skinning](#stage-3) (planned)
    - [Stage 4: Online Multiplayer Mode](#stage-4) (planned)

## Motivation

I used to play [TJAPlayer3](https://github.com/twopointzero/TJAPlayer3), [Taiko Web](https://github.com/bui/taiko-web), and [OpenTaiko](https://github.com/0auBSQ/OpenTaiko). [TJAPlayer3](https://github.com/twopointzero/TJAPlayer3) and [OpenTaiko](https://github.com/0auBSQ/OpenTaiko) both have some bugs I wish I could fix. However, many variables, functions, and classes are named in Japanese, which I don't understand. ~~I'm also too lazy to learn C#.~~ I didn't notice any bug in [Taiko Web](https://github.com/bui/taiko-web), but it has an unacceptable input delay. Hence, I decided to build a player myself.

Why `Rust`? `Rust` is/has:
- statically typed
- strongly typed
- no undefined behavior
- powerful compile-time checking
- powerful `enum` and `match`
- fast
- memory-safe
- thread-safe

## Dependencies
- Linux
    - `pkg-config`
    - `libasound2-dev` (Debian-based) or `alsa-lib-devel` (RHEL-based)
- Windows
    - (none)
- macOS
    - (never tested)

## Road Map

- <span id="stage-1">Stage 1: Visualizing (v0.1.x)</span> **(developing)**

    I plan to visualize the charts just like how it is done in [tja-tools](https://github.com/WHMHammer/tja-tools).

- <span id="stage-0">Stage 0: Parsing (v0.0.x)</span> **(finished)**

    `TNTNT`is able to recognize the following meta data, notes, and commands:
    - Meta (common)
        - `TITLE` (including the `EN` and `CN` variants)
        - `SUBTITLE` (including the `EN` and `CN` variants)
        - `BPM`
        - `WAVE`
        - `OFFSET`
        - `DEMOSTART`
        - `GENRE`
        - `SCOREMODE`
        - `LIFE`
        - `BGMOVIE`
    - Meta (course-specific)
        - `COURSE`
        - `LEVEL`
        - `BALLOON`
        - `SCOREINIT`
        - `SCOREDIFF`
        - `STYLE`
        - `EXAM1`
        - `EXAM2`
        - `EXAM3`
    - Notes
        - `0`-`9`
    - Commands
        - `#START`
        - `#END`
        - `#MEASURE`
        - `#BPMCHANGE`
        - `#DELAY`
        - `#SCROLL`
        - `#GOGOSTART`
        - `#GOGOEND`
        - `#BARLINEOFF`
        - `#BARLINEON`
        - `#BRANCHSTART`
        - `#N`
        - `#E`
        - `#M`
        - `#BRANCHEND`
        - `#SECTION`
        - `#LYRIC`
        - `#LEVELHOLD`
        - `#NEXTSONG`

- <span id="stage-2">Stage 2: Desktop Application (v1.x.y)</span> (planned

    I plan to make a playable desktop application in this stage.

- <span id="stage-3">Stage 3: Skinning (v1.x.y)</span> (planned)

    I plan to add skinning support in this stage.

- <span id="stage-4">Stage 4: Online Multiplayer Mode (v2.x.y)</span> (planned)

    I plan to add an online multiplayer mode in this stage.
