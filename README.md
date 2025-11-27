# omt-experiments
Omni-Mad Tools - Experiments

> **Note:** This repository is archived and exists only for historical reasons. The last update was in January 2020. For the current, actively maintained version, see **[omt](https://github.com/AndreasOM/omt)**.

## Overview

This repository contains experimental Rust implementations of various game development tools. It served as a prototyping sandbox during the migration of Omni-Mad Tools to Rust.

The repository contains 5 numbered test cases:
- **0001-packer** - Basic OMAR archive packer
- **0002-packer-struct** - Enhanced packer with struct-based architecture
- **0003-asset** - Asset build system driven by YAML configuration
- **0004-atlas** - Texture atlas packing and inspection tool
- **0005-soundbank** - Soundbank tool stub

## History

*Note: Some historical details may not be 100% accurate due to incomplete records.*

Omni-Mad Tools has evolved through multiple major rewrites across different programming languages:

1. **Ruby Implementation** (dates unknown) - The original version, now lost to history. Code comments reference "old ruby" behavior that was intentionally replicated for compatibility.

2. **C++ Implementation** (dates unknown) - A second rewrite, also lost to history.

3. **Rust Experimental Phase** (December 7, 2019 - January 6, 2020) - **This repository.** Created as an experimental sandbox with numbered test cases to prototype the Rust implementation.

4. **Rust Production** (December 7, 2019 - present) - The production **[omt](https://github.com/AndreasOM/omt)** repository was created on the same day as this experimental repo and ran in parallel. After one month of experimentation, development consolidated entirely into the production repository. The final commit here states: "Added soundbank stub, before deciding to go OMT from now on."

## Why This Repository Exists

This repository is preserved for historical reference to document the experimental prototyping process during the Rust migration.

**For current development and usage, please refer to [omt](https://github.com/AndreasOM/omt).**
