# To - do List


 - [X] Load asset files from AppData folder
 - [X] Actually save player data
 - [ ] Move battle intro transition to world code
 - [X] Reimplement loading/copyright screen

 - [ ] Player creation state

 - [ ] Replace std::collections::HashMap with hashbrown::HashMap

 - [ ] Add é to font

## World

 - [X] Elevation
 - [ ] Reimplement scripts
 - [ ] Reimplement doors
 - [X] Load more tile and location data from world binary
    - [ ] Finished implementing
 - [ ] Proper NPC rendering based on their widths and heights

 - [X] Reverse render coords for Y-axis so bottom is drawn lower
 - [ ] Fix npcs getting drawn offscreen and map connections being drawn over main map

* Pokemarts and Pokemon centers custom implementation

* Maybe day/night cycle?

## Battle

* Moves use PP in battle
* Persistent moves in battle
* Better AI

## Battle GUI

* Fix Battle GUI sucking
* Fix pokemon not getting updated after battle
* Fix higher level pokemon not doing correct damage in battles
* Reimplement running from battles
* Correctly add BattleCloser to BattleGui
* Move animations

## Engine

* Text line wrapping
* Support for text scrolling

## Other

* Add rhai scripting to world and NPC scripting
* Online battles over discord rich presence?
* Add UNKNOWN pokemon, move and item values
* Town Map item
* Multiplayer