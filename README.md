# TNTNT 

`TNTNT`（**T**NTNT is **N**ot **T**aiko **n**o **T**atsujin，读作“踢—嗯—踢—嗯—剔”）旨在解析`.tja`文件并：
- 为每个难度生成图形化谱面
- 体用一个交互式的界面让用户演奏谱面

`TNTNT`目前仍在开发的极早阶段，尚不能工作。直到发布第一个可玩的原型前我都不会接受外部的代码贡献。[路线图](#路线图)中记录了详细的开发计划和进展。

[English (United States)](README-en_US.md)

## 目录

- [动机](#动机)
- [路线图](#路线图)
    - [第零阶段：解析](#stage-0)**（目前进度）**
    - [第一阶段：图形化谱面](#stage-1)
    - [第二阶段：演奏](#stage-2)
    - [第三阶段：皮肤](#stage-3)
    - [第四阶段：在线多人模式](#stage-4)

## 动机

在开发本项目之前，我曾用过[TJAPlayer3](https://github.com/twopointzero/TJAPlayer3)和[OpenTaiko](https://github.com/0auBSQ/OpenTaiko)。它们有些bug，但我试图修复时发现有许多变量、函数和类都是用日文命名的（我不会日文）。~~我也同样不想学C#。~~[Taiko Web](https://github.com/bui/taiko-web)做得很好，但我无法忍受它的输入延迟。因此，我决定发起`TNTNT`项目，做一个我自己的`.tja`模拟器。

为什么用`Rust`？`Rust`有以下优点：
- 快
- 静态类型
- 强类型
- 强大的枚举类和模式匹配
- 强大的编译期类型推导
- 内存安全
- 线程安全

## 路线图

- <span id="stage-0">第零阶段：解析（v0.x）</span>**（目前进度）** 

    我计划在本阶段识别以下元信息、音符和指令：
    - 元信息（通用）
        - [x] TITLE（包括`EN`和`CN`变体）
        - [x] SUBTITLE（包括`EN`和`CN`变体）
        - [x] BPM
        - [x] WAVE
        - [x] OFFSET
        - [x] DEMOSTART
        - [x] GENRE
        - [x] SCOREMODE
        - [x] LIFE
        - [x] BGMOVIE
    - 元信息（各难度独立）
        - [x] COURSE
        - [x] LEVEL
        - [x] BALLOON
        - [x] SCOREINIT
        - [x] SCOREDIFF
        - [x] STYLE
        - [x] EXAM1
        - [x] EXAM2
        - [x] EXAM3
    - 音符
        - [ ] 0-9
    - 指令
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

- <span id="stage-1">第一阶段：图形化谱面（v1.x）</span>

    我计划在本阶段如[tja-tools](https://github.com/WHMHammer/tja-tools)一样实现铺面的图形化。

- <span id="stage-2">第二阶段：演奏（v2.x）</span>

    我计划在本阶段实现一个可玩的桌面应用程序。

- <span id="stage-3">第三阶段：皮肤（v3.x）</span>

    我计划在本阶段添加皮肤支持。

- <span id="stage-4">第四阶段：在线多人模式（v4.x）</span>

    我计划在本阶段加入在线多人模式。
