use macroquad::miniquad::conf::{AppleGfxApi, Platform};
use macroquad::prelude::*;
use matte::{Align::*, Edge::*, Fitting, Frame, Num, Rect};

// Uses Metal on macOS, since OpenGL flickers while resizing.
// Pass "opengl" as the first argument to use OpenGL instead.
fn window_conf() -> Conf {
    let apple_gfx_api = if std::env::args().nth(1).as_deref() == Some("opengl") {
        AppleGfxApi::OpenGl
    } else {
        AppleGfxApi::Metal
    };
    Conf {
        window_title: "Frame Layout".to_owned(),
        platform: Platform {
            apple_gfx_api,
            ..Default::default()
        },
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
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

        // Macroquad Text params
        let fixed_text = TextParams {
            font_size: 16,
            ..Default::default()
        };

        // Drawing helper function. Defined as a closure so that it can use the current
        // "scale" without passing it as an argument
        let draw_rect = |rect: &Rect<f32>, color: [u8; 4], text: &'_ str| {
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
            draw_text_ex(text, rect.x + t.x, rect.y + t.y, rect_text);
        };

        // Init Layout. Prevents negative values.
        // You can optionally clamp it to  a minimum UI size.
        let mut root = Frame::new(Rect {
            x: 10.0,
            y: 30.0,
            w: (width - 20.0).clamp(0.0, 8192.0),
            // Shorter so I can watch the fitting behavior at the bottom
            h: (height - 20.0).clamp(0.0, 8192.0) * 0.95,
        });
        // root.set_margin(4.0);
        root.set_scale(scale);
        root.fitting = Fitting::Aggressive;

        // Process Layout;
        draw_text_ex(
            "Use '+', '-' and '0' keys to change UI scale",
            10.0,
            16.0,
            fixed_text,
        );
        draw_rect(&root.rect(), [60, 60, 60, 255], "");

        // Left pane;
        root.push_edge(Left, 200.0, |pane| {
            draw_rect(&pane.rect(), [76, 76, 76, 255], "left pane (scaled)");
            pane.set_margin(8.0);
            pane.set_gap(2.0);
            pane.push_edge(Top, 20.0, |_space| {});
            pane.fitting = Fitting::Scale;
            // Buttons
            for n in 0..20 {
                pane.push_edge(Top, 30.0, |button| {
                    // pane.push_size(TopLeft, 200.0, 30.0, |button| {
                    let text = if button.rect().h > 16.0 {
                        format!("button {}", n)
                    } else {
                        format!("")
                    };
                    draw_rect(&button.rect(), [100, 100, 100, 255], text.as_str());
                    button.set_margin(2.0);
                    button.push_edge(Right, 18.0, |icon| {
                        draw_rect(&icon.rect(), [110, 110, 110, 255], "");
                    });
                });
            }
        });

        // Right Pane
        root.push_edge(Right, 200.0, |pane| {
            pane.push_edge(Top, 16.0, |_top_space| {});
            draw_rect(&pane.rect(), [88, 88, 88, 255], "right pane");
            // Buttons
            let count = 20;
            let split_h = pane.divide_height(count); // / pane.get_scale();
            for n in 0..count {
                pane.push_edge(Top, split_h, |button| {
                    let text = format!("resizable button {}", n + 1);
                    draw_rect(&button.rect(), [120, 120, 120, 255], text.as_str());
                });
            }
        });

        // Middle Left
        let split_h = root.divide_width(2);
        root.push_edge(Left, split_h, |pane| {
            pane.fitting = Fitting::Scale;
            pane.set_margin(16.0);
            draw_rect(&pane.rect(), [120, 120, 120, 255], "middle left");
            // Sized rect, will scale down preserving aspect
            pane.push_size(BottomLeft, 100.0, 50.0, |sized| {
                draw_rect(&sized.rect(), [120, 120, 120, 255], "sized");
            });
            pane.push_edge(Top, 50.0, |button| {
                draw_rect(&button.rect(), [120, 120, 120, 255], "");
            });
        });

        // Middle Top
        let split_h = root.divide_height(2);
        root.push_edge(Top, split_h, |pane| {
            draw_rect(&pane.rect(), [130, 130, 130, 255], "middle top");
            let top_space = 16.0;
            pane.push_edge(Top, top_space, |_space| {});
            // Spiral rects!
            for _ in 0..3 {
                let split_h = pane.divide_width(4);
                let split_v = pane.divide_height(4);
                pane.push_edge(Left, split_h, |pane| {
                    draw_rect(&pane.rect(), [160, 160, 160, 255], "t");
                });
                pane.push_edge(Top, split_v, |pane| {
                    draw_rect(&pane.rect(), [160, 160, 160, 255], "r");
                });
                pane.push_edge(Right, split_h, |pane| {
                    draw_rect(&pane.rect(), [160, 160, 160, 255], "b");
                });
                pane.push_edge(Bottom, split_v, |pane| {
                    draw_rect(&pane.rect(), [160, 160, 160, 255], "l");
                });
            }
            // draw_rect(&pane.cursor(), [128, 255, 255, 255], "end");
            pane.fill(|end_area| {
                draw_rect(&end_area.rect(), [220, 220, 220, 255], "end");
            });
        });

        // Middle Bottom
        root.fill(|pane| {
            // pane.fitting = Fitting::Scale; // TODO: Uncommenting causes a sizing bug in "fancy_panel"!
            add_fancy_panel(pane, |area| {
                area.push_edge(Bottom, 20.0, |info| {
                    draw_rect(&info.rect(), [56, 56, 56, 255], "info bar");
                });
                for _ in 0..25 {
                    area.push_edge(Top, 40.0, |button| {
                        draw_rect(&button.rect(), [32, 32, 32, 255], "test");
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
    let bar = 16.0 * frame.get_scale();
    frame.fitting = Fitting::Scale;
    let text_size = 16.0;
    let text_params = TextParams {
        font_size: (text_size * frame.get_scale()) as u16,
        ..Default::default()
    };
    let rect = Rect::new(
        frame.cursor().x.to_f32(),
        frame.cursor().y.to_f32(),
        frame.cursor().w.to_f32(),
        frame.cursor().h.to_f32(),
    );
    let text_offset = Vec2::new(4.0, 12.0) * frame.get_scale();

    let text = "Fancy Custom Panel";
    let text_width = text_size * 0.5 * text.chars().count() as f32 * frame.get_scale();
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
    frame.push_edge(Top, T::from_f32(bar), |_| {});
    frame.fill(|content| func(content));
}
