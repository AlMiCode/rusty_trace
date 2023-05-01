# WIP: Rusty Trace
> A toy editor for experimenting with raytracing, focused on simplicity and extensibility.

## To Do

- [X] Texture Editor
- [] Material Editor
- [] Object Editor
- [] Saving and loading `Scene` to/from file
- [] Support `.obj` models
- [] Add modifiers (rotations, combinations, etc) to Hittables
- [] Document exising code

## Project structure
Here only top structure is shown. We try to organise our code in self-explanatory way, so that it wouldn't need additional comments.
``` text
src/
├── gui/       -- Holds basic Camera model
│   ├── views/ -- All components of the ui
│   ├── mod.rs -- Window and mouse/keyboard interactions
│   └── ...    -- to be refactored
├── render/    -- Rendering logic, Hittables, Materials, Textures, etc.
├── io.rs      -- Open, save and hash image
├── lib.rs     -- Exports other modules
├── main.rs    -- Entry point of the app
└── oidn.rs    -- Integration with OIDN, read further for details
```

## OIDN - OpenImageDenoise
If OpenImageDenoise library is installed on your machine, and `OIDN_DIR` environment variable is set to location of the library, Rusty Trace will use it as a denoiser. Otherwise, denoising is currently not supported.
