use macroquad::prelude::*;
use matte::{Align::*, Edge::*, Fitting, Frame, Rect};

#[macroquad::main("Frame Layout")]
async fn main() {
    loop {
        // Init frame
        clear_background(GRAY);
        let (width, height) = (screen_width(), screen_height());

        // Drawing helper function. Converts Rect types between
        // tato_layout's and macroquad's, then draws it.
        fn draw_rect(rect: &Rect<f32>, thickness: f32) {
            let rect = macroquad::math::Rect::new(rect.x, rect.y, rect.w, rect.h);
            draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, thickness, BLACK);
        }

        // Init Layout. Prevents negative values.
        // You can optionally clamp it to  a minimum UI size.
        let mut root = Frame::new(Rect {
            x: 10.0,
            y: 10.0,
            w: (width - 20.0).clamp(0.0, 8192.0),
            h: (height - 20.0).clamp(0.0, 8192.0),
        });

        // ----------------- Process Layout & Draw -----------------
        root.set_margin(8.0);
        root.set_scale(1.0);
        root.fitting = Fitting::Aggressive;
        // root.fitting = Fitting::Clamp;

        // Root rect
        draw_rect(&root.rect(), 4.0);

        // Left pane
        root.push_edge(Left, 200.0, |pane| {
            pane.fitting = Fitting::Scale;
            draw_rect(&pane.rect(), 3.0);
            pane.set_gap(8.0);
            // Buttons
            for _n in 0..25 {
                pane.push_size(TopLeft, 200.0, 20.0, |button| {
                    draw_rect(&button.rect(), 2.0);
                });
            }
        });

        // Right Pane
        root.push_edge(Right, 200.0, |pane| {
            pane.fitting = Fitting::Scale;
            draw_rect(&pane.rect(), 2.0);
            pane.push_size(TopRight, 50.0, 50.0, |center| {
                draw_rect(&center.rect(), 2.0);
            });
        });

        // Height divided by 4, taking margin and gaps ingto account
        let split_v = root.divide_height(4);

        // Middle Top
        root.push_edge(Top, split_v, |pane| {
            draw_rect(&pane.rect(), 2.0);
            let size = pane.cursor();
            pane.push_size(Center, size.w / 2.0, size.h / 2.0, |center| {
                draw_rect(&center.rect(), 2.0);
            });
        });

        // Middle
        root.push_edge(Top, split_v * 2.0, |pane| {
            pane.fitting = Fitting::Scale;
            draw_rect(&pane.rect(), 4.0);
            // Centered. Notice how 'side' here affects which side the available space
            // will be, in this caseit will be on top (the frame was added "from the bottom")
            pane.push_size(Center, 100.0, 100.0, |centered| {
                draw_rect(&centered.rect(), 2.0);
            });
            // Allows specifying extra offsets, width and height.
            pane.place(BottomLeft, 10.0, 10.0, 100.0, 20.0, |inner| {
                draw_rect(&inner.rect(), 2.0);
            });
        });

        // Middle Bottom
        root.fill(|pane| {
            pane.fitting = Fitting::Scale;
            draw_rect(&pane.rect(), 2.0);
            // Sized rect, will scale down preserving aspect
            pane.push_size(BottomCenter, 80.0, 50.0, |sized| {
                draw_rect(&sized.rect(), 2.0);
            });
        });

        // Present frame
        next_frame().await
    }
}
