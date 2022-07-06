# TNTNT 

中文（中国） [English (United States)](README-en_US.md)

`TNTNT`（**T**NTNT is **N**ot **T**aiko **n**o **T**atsujin，读作“踢嗯踢嗯剔”）旨在解析`.tja`文件并：

- 为每个难度生成图形化谱面
- 提供一个交互式的谱面演奏界面

`TNTNT`目前仍在开发的早期阶段。我在发布第一个桌面应用（[第二阶段](#stage-2)完成）前不会接受外部的代码贡献。

请查看[证书](LICENSE.md)文件（英文）以了解本项目的许可。

注意：
- 证书有非官方中文[翻译](https://jxself.org/translations/gpl-3.zh.shtml)（仅[原证书](LICENSE.md)具有法律效力）
- 部分子目录有它们自己的证书文件。

## 目录

- [动机](#动机)
- [已知问题](#已知问题)
- [路线图](#路线图)
    - [第一阶段：图形化谱面](#stage-1)**（开发中）**
    - [第零阶段：解析](#stage-0)（已完成）
    - [第二阶段：桌面应用](#stage-2)（计划中）
    - [第三阶段：在线多人模式](#stage-3)（计划中）

## 动机

我曾使用过[TJAPlayer3](https://github.com/twopointzero/TJAPlayer3)、[Taiko Web](https://github.com/bui/taiko-web)以及[OpenTaiko](https://github.com/0auBSQ/OpenTaiko)。[TJAPlayer3](https://github.com/twopointzero/TJAPlayer3)和[OpenTaiko](https://github.com/0auBSQ/OpenTaiko)都有些bug，但它们很多地方都是用日文命名的，而我不懂日文。<del>我也懒得学C#。</del>[Taiko Web](https://github.com/bui/taiko-web)没什么bug，但它的输入延迟太高了。因此，我决定自己制作一个模拟器。

为什么用`Rust`？`Rust`有以下优点：

- 静态类型
- 强类型
- 无未定义行为
- 强大的编译期检查
- 强大的枚举类型和模式匹配
- 内存安全
- 线程安全
- 快

## 已知问题

- （暂无）

## 路线图

- <span id="stage-1">第一阶段：图形化谱面（v0.1.x）</span>**（开发中）**

    我计划在本阶段实现谱面的图形化（参考[tja-tools](https://github.com/WHMHammer/tja-tools)）。

- <span id="stage-0">第零阶段：解析（v0.0.x）</span>（已完成）

    `TNTNT`可以识别以下元信息、音符和指令：

    - 元信息（通用）
        - `TITLE`（包括`CN`和`EN`变体）
        - `SUBTITLE`（包括`CN`和`EN`变体）
        - `BPM`
        - `WAVE`
        - `OFFSET`
        - `DEMOSTART`
        - `GENRE`
        - `SCOREMODE`
        - `LIFE`
        - `BGMOVIE`
    - 元信息（各难度独立）
        - `COURSE`
        - `LEVEL`
        - `BigBalloon`
        - `SCOREINIT`
        - `SCOREDIFF`
        - `STYLE`
        - `EXAM1`
        - `EXAM2`
        - `EXAM3`
    - 音符
        - `0`（空）
        - `1`（小咚）
        - `2`（小咔）
        - `3`（大咚）
        - `4`（大咔）
        - `5`（小滚奏开始）
        - `6`（大滚奏开始）
        - `7`（小气球开始）
        - `8`（滚奏/气球结束）
        - `9`（大气球开始）
        - `A`（双人大咚）
        - `B`（双人大咔）
        - `C`（炸弹）
        - `F`（隐藏音符）
        - `G`（紫）
    - 指令
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

- <span id="stage-2">第二阶段：桌面应用（v1.x.y）</span>（计划中）

    我计划在本阶段实现交互式的桌面应用程序。

- <span id="stage-3">第三阶段：在线多人模式（v2.x.y）</span>（计划中）

    我计划在本阶段加入在线多人模式。
