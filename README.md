# Pokemon FireRed Clone

My attempt to code somewhat of a clone of Pokemon FireRed in Rust

Building requires "libx11-dev" "libxi-dev" and "libgl1-mesa-dev" on Linux

## Roadmap

### v0.3.X

- [X] Working Party GUI (its not fully textured or completely functional yet though)
- [X] New asset loading system
    - [X] New pokemon loading system
         - [X] Pokemon cry support
         - [X] Pokemon icon support
- [X] New player saves system
- [X] Reorganized player and map data
- [X] Basic warp transitions
- [X] Conditional (yes/no) actions in scripts
- [ ] NPCs can move by themselves
- [ ] Battles close properly
- [ ] All maps and warps up to Cerulean are added
    - [ ] Pokemon centers up to cerulean work

### v0.4.X

- [ ] Finished Party GUI
- [ ] Fixed all warp transitions
- [ ] Move battle code to separate crates
- [ ] Move world rendering code to seperate crate
- [ ] Battle move scripting
- [ ] Battle animations with scripting
- [ ] All NPCs up to Cerulean are added
- [ ] Player has money

### v0.5.X

- [ ] Touchscreen support
- [ ] Basic player bag
- [ ] Pokemarts
- [ ] All scripts up to Cerulean are added
- [ ] Proper texture animation support
    - [ ] Animated water

### Other planned ideas

 - [ ] Game server