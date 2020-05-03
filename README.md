<!---
WhiteBeam Client

Open source EDR with application whitelisting

Copyright 2020, WhiteBeam Security, Inc.
--->
<p align="center">
<img src="https://raw.githubusercontent.com/gist/noproto/f858188c6201b9a7e4ac99157c2546ba/raw/c254b7b3dc65fe7383c66d74f020e2dc4f15ffb5/WhiteBeamText.svg" alt="WhiteBeam">
<br>
<a href="https://github.com/WhiteBeamSec/WhiteBeam/releases" title="Releases"><img src="https://img.shields.io/github/v/tag/WhiteBeamSec/WhiteBeam.svg?style=for-the-badge&label=release&color=blue" alt="Latest Release"></a>
<a href="https://github.com/WhiteBeamSec/WhiteBeam/blob/master/LICENSE.md" title="License"><img src="https://img.shields.io/badge/LICENSE-CC--BY--NC-blue?style=for-the-badge" alt="CC-BY-NC 4.0 Licensed"></a>
<a href="https://github.com/WhiteBeamSec/WhiteBeam/security/policy" title="Security"><img src="https://img.shields.io/badge/bounty-$5K-green?style=for-the-badge" alt="Bounty $5K"></a>
<a href="https://discord.gg/GYSVqYx" target="_blank" title="Chat"><img src="https://img.shields.io/discord/641744447289294859?style=for-the-badge" alt="Chat"></a>
<br>
Open source EDR with application whitelisting
</p>
<img src="https://raw.githubusercontent.com/gist/noproto/f858188c6201b9a7e4ac99157c2546ba/raw/f34a53aa2fc2ea6c3af8a26af43385719318640f/WhiteBeamShield.svg" alt="WhiteBeam Logo" align="right" />

---

## Features

* Block and detect sophisticated attacks
* Modern cryptography: libsodium for hashing and encryption
* Highly compatible: Development focused on all platforms (incl. legacy) and architectures
* Open source: Audits welcome
* Reviewed by security researchers with combined 100+ years of experience

---

## Installation

### From Binaries

**Important**: Always ensure the downloaded file hash matches official hashes ([How-to](https://github.com/WhiteBeamSec/WhiteBeam/wiki/Verifying-file-hashes)).

| Platform        | URL                                                                 | Hash(es) |
| --------------- | ------------------------------------------------------------------- | -------- |
| Linux (x86_64)  | https://dist.whitebeamsec.com/linux/x86_64/WhiteBeam_latest.tar.gz  | [SHA-256](https://dist.whitebeamsec.com/linux/x86_64/WhiteBeam_latest.SHA256)  |
| Linux (i686)    | https://dist.whitebeamsec.com/linux/i686/WhiteBeam_latest.tar.gz    | [SHA-256](https://dist.whitebeamsec.com/linux/i686/WhiteBeam_latest.SHA256)    |
| Linux (aarch64) | https://dist.whitebeamsec.com/linux/aarch64/WhiteBeam_latest.tar.gz | [SHA-256](https://dist.whitebeamsec.com/linux/aarch64/WhiteBeam_latest.SHA256) |

Install WhiteBeam: `./install`

### From Source (Linux)

1. (Optional) Run tests:
`cargo run test`
2. Compile:
`cargo run build`
3. Install WhiteBeam:
`cargo run install`

---

## How to Use

1. (Optional) Set configuration:
`whitebeam --config`
2. Add permitted applications:
`whitebeam --add /absolute/path/to/command`
3. Enable WhiteBeam:
`whitebeam --enable`

---

## In Action

[![asciicast](https://asciinema.org/a/296135.svg)](https://asciinema.org/a/296135)
