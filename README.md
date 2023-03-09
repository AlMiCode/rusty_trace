# WIP: Raytracer library in rust using sdl2

## Current goal

Create a program with graphical interface, that allows to set up a scene of geometrical objects, render it and display on a screen or save to a hard drive.

## To Do

- [X] Implement ray
- [ ] Implement basic collisions with simple geometry shapes
- [ ] Generate image and save it
- [ ] Display an image on a screen upon completion of rendering

## Project structure

``` text
src/
├── gui.rs     -- Manages sdl and draws scenes
├── lib.rs     -- Holds basic LinAlg types and Ray structure and their implementations
├── main.rs
├── scene.rs   -- Holds information about the scene, the gui struct should get a scene struct to draw
└── shapes.rs  -- Holds the possible shapes, stored in Scene and used by Ray for computation
```
