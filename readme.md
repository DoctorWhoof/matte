# Matte

**_WARNING: Under heavy development, API changes constantly._**
------------------------------------------------------------------------

A minimalist **_immediate mode_** and **_no allocations_** layout library for rectangular elements (Frames), intended for games, embedded systems, and other constrained environments. **Matte does not perform any rendering**, it simply lays out nested rectangles (Frames) with margins and gaps between children.

While easy to use, this approach is very limited and can't create complex layouts! There are a few key functions to work with:

- [Frame::push_edge()] inserts a new frame by pushing any edge inwards by a certain amount
- [Frame::push_size()] to add elements with specific dimensions (with automatic scaling)
- [Frame::place()] for arbitrary positioning
- [Frame::fill()] fills the entire available space with a new frame

For instance, repeatedly pushing a new Frame from the same edge is analogous to using a "Row" or "Column" in a more complex GUI library.

It does not have any knowledge of fancy things like _rendering_ and _input_. Instead, it provides you with closures that are aware of their parent Frame's rectangle and available space, and you do the rendering and input yourself.

It also does not know in advance the size of the children, so you may need to do the math yourself within the closure before adding children, although this is planned to be easier in the future. You can use the [Frame::cursor()] method to check the available space after margin is applied, or [Frame::rect()] to get the closure's rectangle.

To evenly divide a frame into columns and rows, you can use the [Frame::divide_width()] and [Frame::divide_height()] functions to obtain the desired width and height of each child frame taking gaps and margin into account.

![LayframeScreenshot](screenshots/screenshot.png)

Three examples are provided:

- _example_basic_, A very simple example using Macroquad for rect rendering.
- _example_mquad_, a more complex layout using Macroquad with text and adjustable zoom.
- _example_sdl_, which uses SDL2 (via the mini_sdl crate).

## Usage Example

```rust
use matte::{Frame, Rect, Fitting, Edge::*, Align::*};

fn main() {
    // Create a root frame
    let mut root = Frame::new(Rect {
        x: 0,
        y: 0,
        w: 800,
        h: 600,
    });

    // Optionally set fitting mode to automatically scale elements
    root.fitting = Fitting::Scale;

    // Add a header at the top
    root.push_edge(Top, 100, |header| {
        // Add a logo to the left of the header
        header.push_edge(Left, 200, |logo| {
            // You can acquire this rectangle using logo.rect(),
            // and draw it with your favorite graphics crate.
        });

        // Add navigation buttons to the right
        header.push_edge(Right, 200, |nav| {
            // Navigation content
            for _ in 0..10 {
                nav.push_edge(Top, 40, |button| {
                    // This "button" is a smaller rect within nav, stacked from the top
                })
            }
        });
    });

    // Add a sidebar with specific width
    root.push_edge(Left, 200, |sidebar| {
        // Sidebar content
    });

    // Add a centered element with specific dimensions (will scale if needed)
    root.push_size(Center, 300, 200, |centered_element| {
        // Centered content that will scale to fit if necessary
    });

    // Main content area (fill remaining space)
    root.fill(|content| {
        // Place an element at specific coordinates with automatic scaling
        content.place(LeftTop, 50, 50, 400, 300, |placed_element| {
            // Element content that scales to fit available space
        });
    });
}
```
