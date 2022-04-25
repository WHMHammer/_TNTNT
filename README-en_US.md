# TNTNT 

`TNTNT` (**T**NTNT is **N**ot **T**aiko **n**o **T**atsujin, pronounced as "tee-en-tee-en-tee") aims to parse `.tja` files and:
- visualize per-course charts
- provide an interactive graphical interface for the users to play the charts

`TNTNT` is currently in its very early stage, not functional at all. I will not accept any external code contribution until I release the first playable prototype. Read the [Road Map](#road-map) for more details.

[中文（中国）](README.md)

## Table of Contents

- [Motivation](#motivation)
- [Road Map](#road-map)
    - [Stage 0: Parsing (v0.x)](#stage-0) **(I am here)**
    - [Stage 1: Visualizing](#stage-1)
    - [Stage 2: Playing](#stage-2)
    - [Stage 3: Skinning](#stage-3)
    - [Stage 4: Online Multiplayer Mode](#stage-4)

## Motivation

I used to play [TJAPlayer3](https://github.com/twopointzero/TJAPlayer3) and [OpenTaiko](https://github.com/0auBSQ/OpenTaiko). There were some bugs I wished I could fix. However, many variables, functions, and classes are named in Japanese, which I don't understand. ~~I don't want to learn C# neither.~~ [Taiko Web](https://github.com/bui/taiko-web) is really nice, but it has an unacceptable input delay. Thus, I decided to start the `TNTNT` project to implement a `.tja` player myself.

Why `Rust`? `Rust` is/has:
- fast
- statically typed
- strongly typed
- powerful `enum` and `match`
- powerful compile-time type inference
- memory-safe
- thread-safe

## Road Map

- <span id="stage-0">Stage 0: Parsing (v0.x)</span> **(I am here)**

    I intend to recognize the following meta data, notes, and commands in this stage:
    - Meta (common)
        - [x] TITLE (including the `EN` and `CN` variants)
        - [x] SUBTITLE (including the `EN` and `CN` variants)
        - [x] BPM
        - [x] WAVE
        - [x] OFFSET
        - [x] DEMOSTART
        - [x] GENRE
        - [x] SCOREMODE
        - [x] LIFE
        - [x] BGMOVIE
    - Meta (course-specific)
        - [x] COURSE
        - [x] LEVEL
        - [x] BALLOON
        - [x] SCOREINIT **(still need to find out the correct default value)**
        - [x] SCOREDIFF **(still need to find out the correct default value)**
        - [x] STYLE
        - [x] EXAM1
        - [x] EXAM2
        - [x] EXAM3
    - Notes
        - [ ] 0-9
    - Commands
        - [ ] #START
        - [x] #END
        - [ ] #MEASURE
        - [ ] #BPMCHANGE
        - [ ] #DELAY
        - [ ] #SCROLL
        - [ ] #GOGOSTART
        - [ ] #GOGOEND
        - [ ] #BARLINEOFF
        - [ ] #BARLINEON
        - [ ] #BRANCHSTART
        - [ ] #N
        - [ ] #E
        - [ ] #M
        - [ ] #BRANCHEND
        - [ ] #SECTION
        - [ ] #LYRIC
        - [ ] #LEVELHOLD
        - [ ] #NEXTSONG

- <span id="stage-1">Stage 1: Visualizing (v1.x)</span>

    I plan to visualize the charts just like how it is done in [tja-tools](https://github.com/WHMHammer/tja-tools).

- <span id="stage-2">Stage 2: Playing (v2.x)</span>

    I plan to make a playable desktop application in this stage.

- <span id="stage-3">Stage 3: Skinning (v3.x)</span>

    I plan to add skinning support in this stage.

- <span id="stage-4">Stage 4: Online Multiplayer Mode (v4.x)</span>

    I plan to add an online multiplayer mode in this stage.
