use macroquad::prelude::*;
use matte::{Fitting, Frame, Num, Side::*};

#[macroquad::main("Frame Layout")]
async fn main() {
    let mut scale: f32 = 1.0;

    loop {
        // Init frame
        clear_background(BLACK);
        let (width, height) = (screen_width(), screen_height());

        // Input
        if is_key_pressed(KeyCode::Equal) {
            scale += 0.1;
        } else if is_key_pressed(KeyCode::Minus) {
            scale -= 0.1;
        } else if is_key_pressed(KeyCode::Key0) {
            scale = 1.0;
        }
        scale = scale.clamp(0.2, 2.0);
        let fixed_text = TextParams {
            font_size: 16,
            ..Default::default()
        };

        // Drawing helper function. Defined as a closure so that it can use the current
        // "scale" without passing it as an argument
        let draw_rect = |rect: &matte::Rect<f32>, color: [u8; 4], text: String| {
            let rect_text = TextParams {
                font_size: (16.0 * scale) as u16,
                ..Default::default()
            };
            let rect = macroquad::math::Rect::new(
                rect.x as f32,
                rect.y as f32,
                rect.w as f32,
                rect.h as f32,
            );
            let t = Vec2::new(4.0, 12.0) * scale;
            draw_rectangle(rect.x, rect.y, rect.w, rect.h, color.into());
            draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 2.0, [0, 0, 0, 128].into());
            draw_text_ex(text.as_str(), rect.x + t.x, rect.y + t.y, rect_text);
        };

        // Init Layout. Prevents negative values.
        // You can optionally clamp it to  a minimum UI size.
        let mut root = Frame::new(matte::Rect {
            x: 10.0,
            y: 30.0,
            w: (width - 20.0).clamp(0.0, 8192.0),
            // Shorter so I can watch the fitting behavior at the bottom
            h: (height - 20.0).clamp(0.0, 8192.0) * 0.95,
        });
        // root.set_margin(4.0);
        root.set_scale(scale);

        // Process Layout;
        draw_text_ex("Use '+', '-' and '0' keys to change UI scale", 10.0, 16.0, fixed_text);
        draw_rect(&root.rect(), [60, 60, 60, 255], "".to_string());


        // Left pane
        root.push(Left, 200.0, |pane| {
            draw_rect(&pane.rect(), [76, 76, 76, 255], "left pane (scaled fitting)".to_string());
            pane.set_margin(8.0);
            pane.set_gap(1.0);
            pane.push(Top, 20.0, |_space| {});
            pane.fitting = Fitting::Scale;
            // Buttons
            for n in 0..20 {
                pane.push_size(Top, pane.cursor().w, 30.0, |button| {
                    button.fitting = Fitting::Scale;
                    let text = if button.rect().h > 16.0 {
                        format!("button {}", n)
                    } else {
                        "".to_string()
                    };
                    draw_rect(&button.rect(), [100, 100, 100, 255], text);
                    button.set_margin(2.0);
                    button.push(Right, 18.0, |icon| {
                        draw_rect(&icon.rect(), [110, 110, 110, 255], "".to_string());
                    });
                });
            }
        });

        // Right Pane
        root.push(Right, 200.0, |pane| {
            draw_rect(&pane.rect(), [88, 88, 88, 255], "right pane".to_string());
            let top_space = 16.0;
            pane.push(Top, top_space, |_space| {});
            let count = 20;
            let gap_sum = (pane.margin() * 2.0) + (pane.gap() * count as f32) + top_space;
            // Available space / count, but I subtract 1.0 to make it more stable
            // when resizing (avoids occasionally skipping last element)
            let button_size = (pane.rect().h - gap_sum - 1.0) / count as f32;

            for n in 0..count {
                pane.push(Top, button_size, |button| {
                    let text = format!("resizable button {}", n + 1);
                    draw_rect(&button.rect(), [120, 120, 120, 255], text);
                });
            }
        });

        // Middle Left
        root.fill(Left, 0.25, |pane| {
            draw_rect(&pane.rect(), [120, 120, 120, 255], "middle left".to_string());
            // Sized rect, will scale down preserving aspect
            pane.push_size(Bottom, 100.0, 50.0, |sized|{
                draw_rect(&sized.rect(), [120, 120, 120, 255], "sized".to_string());
            });
        });

        // Middle Top
        root.fill(Top, 0.5, |pane| {
            draw_rect(&pane.rect(), [130, 130, 130, 255], "middle top".to_string());
            let top_space = 16.0;
            pane.push(Top, top_space, |_space| {});
            // Spiral rects!
            let ratio = 0.3;
            for _ in 0..3 {
                pane.fill(Top, ratio, |pane| {
                    draw_rect(&pane.rect(), [160, 160, 160, 255], "t".to_string());
                });
                pane.fill(Right, ratio, |pane| {
                    draw_rect(&pane.rect(), [160, 160, 160, 255], "r".to_string());
                });
                pane.fill(Bottom, ratio, |pane| {
                    draw_rect(&pane.rect(), [160, 160, 160, 255], "b".to_string());
                });
                pane.fill(Left, ratio, |pane| {
                    draw_rect(&pane.rect(), [160, 160, 160, 255], "l".to_string());
                });
            }
            pane.fill(Left, 1.0, |pane| {
                draw_rect(&pane.rect(), [220, 220, 220, 255], "end".to_string());
            });
        });

        // Middle Bottom
        root.fill(Bottom, 1.0, |pane| {
            pane.fitting = Fitting::Scale;
            add_fancy_panel(pane, |area| {
                area.push(Bottom, 20.0, |button| {
                    draw_rect(&button.rect(), [56, 56, 56, 255], "info bar".to_string());
                });
                for _ in 0..25 {
                    area.push(Top, 40.0, |button| {
                        draw_rect(&button.rect(), [32, 32, 32, 255], "test".to_string());
                    });
                }
            });
        });

        // Present frame
        next_frame().await
    }
}

fn add_fancy_panel<T>(frame: &mut Frame<T>, mut func: impl FnMut(&mut Frame<T>))
where
    T: Num,
{
    frame.fitting = Fitting::Scale;
    let text_size = 16.0;
    let text_params = TextParams {
        font_size: (text_size * frame.scale()) as u16,
        ..Default::default()
    };
    let rect = Rect::new(
        frame.cursor().x.to_f32(),
        frame.cursor().y.to_f32(),
        frame.cursor().w.to_f32(),
        frame.cursor().h.to_f32(),
    );
    let text_offset = Vec2::new(4.0, 12.0) * frame.scale();
    let bar = 16.0 * frame.scale();
    let text = "Fancy Custom Panel";
    let text_width = text_size * 0.5 * text.chars().count() as f32 * frame.scale();
    if text_width < rect.w {
        draw_rectangle(
            rect.x,
            rect.y + bar,
            rect.w,
            rect.h - bar,
            [22, 22, 22, 255].into(),
        );
        draw_rectangle(
            rect.x,
            rect.y,
            text_width + text_offset.x,
            rect.h,
            [22, 22, 22, 255].into(),
        );
        draw_text_ex(
            text,
            rect.x + text_offset.x,
            rect.y + text_offset.y,
            text_params,
        );
    }
    frame.push(Top, T::from_f32(bar), |_| {});
    frame.fill(Top, 1.0, |content| func(content));
}
