# What is this?

This is just a simple rust program that I've started toying with.   It uses the bevy game engine framework.

Currently, it only consists of a players dragon trying to survive against 3 NPC dragons.   The player controls a Fire Dragon.  The three NPC dragons are; a Rock Dragon, an Ice Dragon and a Water Dragon.  The NPC dragons have less speed and shooting power than the players dragon - at least while I'm trying to work it all out.  I've also given the characters dragon 100 health, while the NPCs only get 5 health each!   Each shot takes off one health point for now.

It is still in very, very early proof-of-concept stage, so it's not like it will be super fun yet.

It should run on any platform that rust rust in - even in web-assembly.   
You can view a sample of the game running here;
(createng.com/dragons)[https://createng.com/dragons/]

Controls are still very much a work in progress.  

* If you have a keyboard, you can use W, A, S, and D,  or the arrow keys, to move the dragon. And, the spacebar shoots.
Note, shooting is both offensive and defensive, ie, fire-balls cancel out ice-balls etc.  Oh, and you can hold shift as a brake, for when you want to crawl slowly.

* If you have a mouse, you can use the main left button to shoot, while using the keyboard to move.  Also, if you double click the mouse, you can use it to move, but then you cant shoot at the same time with the mouse until you stop moving.

* If you have a touch-screen - the pale-blue circle thingy in the bottom left corner is a virtual-joystick to move your dragon.   Shooting works just by touching anywhere else on the screen, outside the joystick area.

The shooting direction is also affected by your velocity, so if you have trouble with that, try not moving while shooting and it should more a lot more accurate.

Thats about all there is to it for now.


## To Build and Run on Windows or *nix
`cargo build`

`cargo run`
 
 etc.

## To Build for WebAssembly

`cargo run --bin wasm_build --features wasm_build`

This cargo bin target simply runs the contents of the wasm_build.script file - and then does some md5 hashing to make the webassembly thing avoid using old cached versions.   

At the moment, this works for all desktop platforms I am aware of.

```
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-name wasm_jastery --out-dir wasm/target --target web target/wasm32-unknown-unknown/release/jastery.wasm
```

If you use this software, and it eats your cat, well, that is exactly what dragons are likely to do, so please dont blame me.

All of the artwork, source code and game content was developed by me - so I own the copyright.   Not sure what type license it should go under yet, maybe something like MIT or Apache sort of thing.
