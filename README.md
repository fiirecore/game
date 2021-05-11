# Pokemon FireRed Clone

My attempt to code somewhat of a clone of Pokemon FireRed in Rust

Building requires "libx11-dev" "libxi-dev" and "libgl1-mesa-dev" on Linux

## Roadmap

### v0.3.0

- [X] Working Party GUI (its not fully textured or completely functional yet though)
- [X] New asset loading system
    - [X] New pokemon loading system
         - [X] Pokemon cry support
         - [X] Pokemon icon support
- [X] New player saves system
    - [X] New main menu
- [X] Reorganized player and map data
- [X] Basic warp transitions
- [X] Conditional (yes/no) actions in scripts
- [X] NPCs can move by themselves
- [X] All maps and warps up to Cerulean are added
    - [X] Pokemon centers up to cerulean work
- [X] Battles close properly

- [X] Party GUI select menu
- [X] Basic party GUI summary gui

### v0.4.0

- [X] Independent world random, battle random, and wild random
- [ ] Finished Party GUI
- [ ] Fixed all warp transitions
- [X] Move battle code to separate crates
- [X] Move world rendering code to seperate crate
- [X] Battle move scripting
- [X] Battle animations with scripting
- [ ] All NPCs up to Cerulean are added
- [X] Player has money
- [ ] Battle AI

### v0.5.0

- [ ] Touchscreen support
- [X] Basic player bag
- [ ] Pokemarts
- [ ] All scripts up to Cerulean are added
- [ ] Proper texture animation support
    - [ ] Animated water

### Other planned ideas

 - [ ] Game server