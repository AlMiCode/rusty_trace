# WIP: Raytracer library in rust using sdl2

## RADICAL !!!
DO NOT CONFUSE WITH `main` BRANCH!!!

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
├── camera.rs    -- Holds basic Camera model
├── gui.rs       -- Manages sdl and draws images
├── lib.rs       -- Holds basic LinAlg types, Ray struct and core rendering logic 
├── main.rs
└── hittable.rs  -- Holds the Hittable trait and its implementations
```
