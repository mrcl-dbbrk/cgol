# cgol

**DISCLAIMER**: You probably should not rely on this software at this point. I'm just trying things out here.

This is yet another Conway's Game of Life, written in Rust.

## Controls

| Input               | Effect            |
| :-----------------: | :---------------: |
| Left Mouse-Button   | Add Cell          |
| Right Mouse-Button  | Remove Cell       |
| Middle Mouse-Button | Pan Around        |
| Space-Key           | Pause/Un-Pause    |
| C-Key               | Clear Cells       |
| F-Key               | Toggle Fullscreen |

## Bugs

Some issues and bugs mostly caused by me having been a little lazy ;-). I'm gonna try find some time to fix them:
 - Zooming happens relative to world origin rather than to the center of the window or relative to mouse position.
 - All Quadrants, but the one to the bottom right of world origin, are off by a pixel.
 - Probably not really a bug, but proper line drawing might be nice (I'm considering Bresenham).
