#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/readme.md"))]
use core::marker::PhantomData;

mod rect;
pub use rect::*;

mod num;
pub use num::{Float, Integer, Num, SignedNum};

mod vec2;
pub use vec2::*;

/// Shortens signature for a mutable frame reference
macro_rules! child {
    () => {
        impl FnMut(&mut Frame<T>)
    };
}

/// A layout frame that manages rectangular areas with margins and scaling.
/// A frame consists of an outer rectangle, an inner cursor rectangle (available space),
/// and properties that control how child frames are created and positioned.
#[derive(Debug, Clone)]
pub struct Frame<T> {
    /// The outer rectangle defining the frame boundaries (f32 internally)
    rect: Rect<f32>,
    /// Inner rectangle representing available space (f32 internally)
    cursor: Rect<f32>,
    /// Scaling factor for dimensions
    scale: f32,
    /// Margin size between frames (f32 internally)
    margin: f32,
    /// Gap between each child frame (f32 internally)
    gap: f32,
    /// Controls how children rects are culled when they exceed available space
    pub fitting: Fitting,
    /// Phantom data to retain type parameter T
    _phantom: PhantomData<T>,
}

/// Represents the side of a frame where a child frame can be added.
#[derive(Debug, Clone, Copy, Default)]
pub enum Edge {
    #[default]
    /// Left side of the frame
    Left,
    /// Right side of the frame
    Right,
    /// Top side of the frame
    Top,
    /// Bottom side of the frame
    Bottom,
}

/// Represents the alignment of a child frame that is sized (width, height).
/// Notice that LeftTop is *not* the same as TopLeft! LeftTop means "push_edge from the left,
/// align to Top" and TopLeft means "push_edge from Top, align to Left". The result may look the same,
/// but the available space will shrink from the left in the former, from the top in the latter.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum Align {
    #[default]
    LeftTop,
    LeftCenter,
    LeftBottom,
    RightTop,
    RightCenter,
    RightBottom,
    TopLeft,
    TopCenter,
    TopRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
    /// Only option that does not shrink the available space (the "cursor" rect).
    Center,
}

/// Clipping strategy
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Fitting {
    /// Allows a child frame even if it goes over the available space.
    /// Also useful for debugging, since Frame is less likely to disappear when space is too small.
    Relaxed,
    /// Removes child frames that touch the margin.
    Aggressive,
    /// Clamps child frame's edges to available space.
    Clamp,
    /// Scales the child frame to fit available space while preserving aspect ratio.
    Scale,
}

impl<T> Frame<T>
where
    T: Num,
{
    /// Creates a new frame with the specified outer rectangle.
    /// Initializes with default values for scale (1.0) and margin (4 units).
    pub fn new(rect: Rect<T>) -> Self {
        let scale = 1.0;
        let margin = 4.0;
        let rect = rect.to_f32();
        let cursor = rect.shrink(margin);

        Self {
            rect,
            cursor,
            margin,
            gap: margin,
            scale,
            fitting: Fitting::Aggressive,
            _phantom: PhantomData,
        }
    }

    /// The rect that represents this Frame's position and size.
    /// Does not change when adding child frames.
    pub fn rect(&self) -> Rect<T> {
        Rect::<T>::from_f32(self.rect)
    }

    /// The available space to add more child frames.
    /// Shrinks every time a child frame is added.
    pub fn cursor(&self) -> Rect<T> {
        Rect::<T>::from_f32(self.cursor)
    }

    /// Returns the current margin value.
    pub fn get_margin(&self) -> T {
        T::from_f32(self.margin)
    }

    /// Sets a new margin value and recalculates the cursor rectangle.
    pub fn set_margin(&mut self, margin: T) {
        // Remove old margin
        self.cursor = self.cursor.expand(self.margin);
        // Apply new margin
        self.margin = margin.to_f32();
        self.cursor = self.rect.shrink(self.margin);
    }

    /// Returns the current gap value.
    pub fn get_gap(&self) -> T {
        T::from_f32(self.gap)
    }

    /// Sets a new gap value.
    pub fn set_gap(&mut self, gap: T) {
        self.gap = gap.to_f32();
    }

    /// Returns the current scale factor.
    pub fn get_scale(&self) -> f32 {
        self.scale
    }

    /// Sets a new scale factor for the frame.
    pub fn set_scale(&mut self, scale: f32) {
        self.scale = scale;
    }

    /// Calculates the size if you divide the available space's width by "columns",
    /// taking into account the size of the gaps between each column.
    /// The number of columns stays consistent regardless of scale.
    pub fn divide_width(&self, columns: u32) -> T {
        if columns <= 1 {
            return T::from_f32(self.cursor.w);
        }
        let unscaled_gap = self.gap * (columns - 1) as f32;
        let available_width = (self.cursor.w / self.scale) - unscaled_gap;
        let size = available_width / columns as f32;
        T::from_f32(size)
    }

    /// Calculates the size if you divide the available space's height by "rows",
    /// taking into account the size of the gaps between each row.
    /// The number of rows stays consistent regardless of scale.
    pub fn divide_height(&self, rows: u32) -> T {
        if rows <= 1 {
            return T::from_f32(self.cursor.h);
        }
        let unscaled_gap = self.gap * (rows - 1) as f32;
        let available_height = (self.cursor.h / self.scale) - unscaled_gap;
        let size = available_height / rows as f32;
        T::from_f32(size)
    }

    /// Determines the edge associated with an alignment.
    fn alignment_to_edge(align: Align) -> Edge {
        match align {
            Align::LeftTop | Align::LeftCenter | Align::LeftBottom => Edge::Left,
            Align::RightTop | Align::RightCenter | Align::RightBottom => Edge::Right,
            Align::TopLeft | Align::TopCenter | Align::TopRight => Edge::Top,
            Align::BottomLeft | Align::BottomCenter | Align::BottomRight => Edge::Bottom,
            Align::Center => Edge::Left, // Center uses left as base for positioning
        }
    }

    /// Converts an edge to its default alignment.
    fn edge_to_alignment(edge: Edge) -> Align {
        match edge {
            Edge::Left => Align::LeftTop,
            Edge::Right => Align::RightTop,
            Edge::Top => Align::TopLeft,
            Edge::Bottom => Align::BottomLeft,
        }
    }

    /// Calculates offsets based on alignment for positioned frames.
    fn calculate_align_offsets(&self, align: Align, w: f32, h: f32) -> (f32, f32) {
        let (offset_x, offset_y) = match align {
            // Left edge alignments
            Align::LeftTop => (0.0, 0.0),
            Align::LeftCenter => (0.0, (self.cursor.h - h) / 2.0),
            Align::LeftBottom => (0.0, (self.cursor.h - h).max(0.0)),

            // Right edge alignments
            Align::RightTop => (0.0, 0.0),
            Align::RightCenter => (0.0, (self.cursor.h - h) / 2.0),
            Align::RightBottom => (0.0, (self.cursor.h - h).max(0.0)),

            // Top edge alignments
            Align::TopLeft => (0.0, 0.0),
            Align::TopCenter => ((self.cursor.w - w) / 2.0, 0.0),
            Align::TopRight => ((self.cursor.w - w).max(0.0), 0.0),

            // Bottom edge alignments
            Align::BottomLeft => (0.0, 0.0),
            Align::BottomCenter => ((self.cursor.w - w) / 2.0, 0.0),
            Align::BottomRight => ((self.cursor.w - w).max(0.0), 0.0),

            // Center alignment
            Align::Center => ((self.cursor.w - w) / 2.0, (self.cursor.h - h) / 2.0),
        };

        // Ensure offsets are non-negative
        let x = offset_x.max(0.0);
        let y = offset_y.max(0.0);

        (x, y)
    }

    /// Calculates the scale needed to fit a rectangle of given dimensions
    /// into the available space, preserving aspect ratio.
    /// Takes into account the offsets where the rectangle will be placed.
    fn calculate_fit_scale(&self, w: f32, h: f32, offset_x: f32, offset_y: f32) -> f32 {
        match self.fitting {
            Fitting::Relaxed | Fitting::Aggressive | Fitting::Clamp => self.scale,
            Fitting::Scale => {
                if w <= 0.0 || h <= 0.0 {
                    return self.scale;
                }

                // Calculate available space considering offsets
                let available_w = self.cursor.w - offset_x;
                let available_h = self.cursor.h - offset_y;

                if available_w <= 0.0 || available_h <= 0.0 {
                    return self.scale;
                }

                // Calculate scale ratios for width and height
                let scale_w = available_w / w;
                let scale_h = available_h / h;

                // Use the smaller ratio to maintain aspect ratio
                let fit_scale = scale_w.min(scale_h);

                // Apply base scale but cap it to fit in available space
                if self.scale >= 1.0 {
                    self.scale.min(fit_scale)
                } else {
                    self.scale.min(fit_scale)
                }
            }
        }
    }

    /// Attempts to add a frame with the specified size (w,h).
    /// Does not modify the available space if Align is Center.
    /// # Parameters
    /// * `align` - Alignment that determines positioning and cursor updating
    /// * `w` - Width of the new frame
    /// * `h` - Height of the new frame
    /// * `func` - Closure to execute with the new child frame
    #[inline(always)]
    pub fn push_size(&mut self, align: Align, w: T, h: T, func: child!()) {
        let w_f32: f32 = w.to_f32();
        let h_f32: f32 = h.to_f32();
        let (offset_x, offset_y) = self.calculate_align_offsets(align, w_f32, h_f32);
        let edge = Self::alignment_to_edge(align);

        // Calculate actual scale for Fitting::Scale
        let actual_scale: f32 = self.calculate_fit_scale(w_f32, h_f32, offset_x, offset_y);

        // Final offsets with actual size
        let scaled_w: f32 = w_f32 * actual_scale;
        let scaled_h: f32 = h_f32 * actual_scale;
        let (offset_x, offset_y) = self.calculate_align_offsets(align, scaled_w, scaled_h);

        let update_cursor = align != Align::Center;

        self.add_scope(
            edge,
            offset_x,
            offset_y,
            w_f32,
            h_f32,
            actual_scale,
            update_cursor,
            self.fitting,
            func,
        );
    }

    /// Adds a new frame on the specified edge with specified length.
    /// # Parameters
    /// * `edge` - Which edge to add the child frame to
    /// * `len` - Length of the new frame
    /// * `func` - Closure to execute with the new child frame
    #[inline(always)]
    pub fn push_edge(&mut self, edge: Edge, len: T, func: child!()) {
        let len_f32: f32 = len.to_f32();
        // Default width and height based on the edge
        let is_horizontal = matches!(edge, Edge::Left | Edge::Right);
        let (w_f32, h_f32): (f32, f32) = if is_horizontal {
            (len_f32, self.cursor.h / self.scale)
        } else {
            (self.cursor.w / self.scale, len_f32)
        };

        let align = Self::edge_to_alignment(edge);
        let (offset_x, offset_y) = self.calculate_align_offsets(align, w_f32, h_f32);
        let actual_scale: f32 = self.calculate_fit_scale(w_f32, h_f32, offset_x, offset_y);
        let update_cursor = align != Align::Center;

        self.add_scope(
            edge,
            offset_x,
            offset_y,
            w_f32,
            h_f32,
            actual_scale,
            update_cursor,
            self.fitting,
            func,
        );
    }

    /// Fills the entire available cursor area with a new Frame.
    /// # Parameters
    /// * `func` - Closure to execute with the new child frame
    pub fn fill(&mut self, func: child!()) {
        self.add_scope(
            Edge::Top,
            0.0,
            0.0,
            self.cursor.w,
            self.cursor.h,
            1.0,
            true,
            self.fitting,
            func,
        );
    }

    /// Allows arbitrary placement of the new frame in relation to the current frame.
    /// Does not modify the available space if Align is Center.
    /// Scales the frame if necessary to fit.
    /// # Parameters
    /// * `align` - Alignment that determines cursor updating
    /// * `x` - X position of the new frame in relation to this frame
    /// * `y` - Y position of the new frame in relation to this frame
    /// * `w` - Width of the new frame
    /// * `h` - Height of the new frame
    /// * `func` - Closure to execute with the new child frame
    pub fn place(&mut self, align: Align, x: T, y: T, w: T, h: T, func: child!()) {
        let x_f32: f32 = x.to_f32();
        let y_f32: f32 = y.to_f32();
        let w_f32: f32 = w.to_f32();
        let h_f32: f32 = h.to_f32();
        let edge = Self::alignment_to_edge(align);
        let update_cursor = align != Align::Center;

        // Calculate actual scale and apply it to dimensions, taking offsets into account
        let actual_scale: f32 = self.calculate_fit_scale(w_f32, h_f32, x_f32, y_f32);

        self.add_scope(
            edge,
            x_f32,
            y_f32,
            w_f32,
            h_f32,
            actual_scale,
            update_cursor,
            self.fitting,
            func,
        );
    }

    /// Internal multi-purpose function called by the mode-specialized public functions.
    fn add_scope(
        &mut self,
        edge: Edge,
        extra_x: f32,
        extra_y: f32,
        w: f32,
        h: f32,
        scale: f32,
        update_cursor: bool,
        fitting: Fitting,
        mut func: child!(),
    ) {
        let scaled_w = w * scale;
        let scaled_h = h * scale;
        let margin = self.gap * self.scale;
        let gap = self.gap * self.scale;

        if scaled_w < 1.0 || scaled_h < 1.0 {
            return;
        }

        // Calculate the child rectangle based on the edge
        let mut child_rect_f32: Rect<f32> = match edge {
            Edge::Left => {
                if self.cursor.x > self.rect.x + self.rect.w {
                    return;
                }
                Rect::new(
                    self.cursor.x + extra_x,
                    self.cursor.y + extra_y,
                    scaled_w,
                    scaled_h,
                )
            }
            Edge::Right => Rect::new(
                (self.cursor.x + self.cursor.w - scaled_w).max(0.0) - extra_x,
                self.cursor.y + extra_y,
                scaled_w,
                scaled_h,
            ),
            Edge::Top => {
                if self.cursor.y > self.rect.y + self.rect.h {
                    return;
                }
                Rect::new(
                    self.cursor.x + extra_x,
                    self.cursor.y + extra_y,
                    scaled_w,
                    scaled_h,
                )
            }
            Edge::Bottom => Rect::new(
                self.cursor.x + extra_x,
                (self.cursor.y + self.cursor.h - scaled_h).max(0.0) - extra_y,
                scaled_w,
                scaled_h,
            ),
        };

        if child_rect_f32.x > self.cursor.x + self.cursor.w - self.margin {
            return;
        }

        if child_rect_f32.y > self.cursor.y + self.cursor.h - self.margin {
            return;
        }

        match fitting {
            Fitting::Relaxed => {}
            Fitting::Aggressive => {
                if (child_rect_f32.x + child_rect_f32.w) as i32 as f32
                    > (self.cursor.x + self.cursor.w) as i32 as f32 + 1.0
                {
                    return;
                }
                if (child_rect_f32.y + child_rect_f32.h) as i32 as f32
                    > (self.cursor.y + self.cursor.h) as i32 as f32 + 1.0
                {
                    return;
                }
            }
            Fitting::Clamp => {
                // Clamp to ensure the rect stays within cursor boundaries
                // Clamp x position
                if child_rect_f32.x < self.cursor.x {
                    let diff = self.cursor.x - child_rect_f32.x;
                    child_rect_f32.x = self.cursor.x;
                    child_rect_f32.w = (child_rect_f32.w - diff).max(0.0);
                }

                // Clamp y position
                if child_rect_f32.y < self.cursor.y {
                    let diff = self.cursor.y - child_rect_f32.y;
                    child_rect_f32.y = self.cursor.y;
                    child_rect_f32.h = (child_rect_f32.h - diff).max(0.0);
                }

                // Clamp width
                if child_rect_f32.x + child_rect_f32.w > self.cursor.x + self.cursor.w {
                    child_rect_f32.w = self.cursor.x + self.cursor.w - child_rect_f32.x;
                }

                // Clamp height
                if child_rect_f32.y + child_rect_f32.h > self.cursor.y + self.cursor.h {
                    child_rect_f32.h = self.cursor.y + self.cursor.h - child_rect_f32.y;
                }
            }
            Fitting::Scale => {
                // The scaling is now handled prior to this function in the calling methods
            }
        }

        if child_rect_f32.w < 1.0 || child_rect_f32.h < 1.0 {
            return;
        }

        // Update parent cursor
        if update_cursor {
            match edge {
                Edge::Left => {
                    // Add extra_x to the cursor movement
                    self.cursor.x += scaled_w + gap + extra_x;
                    self.cursor.w = (self.cursor.w - scaled_w - gap - extra_x).max(0.0);
                }
                Edge::Right => {
                    // Subtract extra_x in width reduction
                    self.cursor.w = (self.cursor.w - scaled_w - gap - extra_x).max(0.0);
                }
                Edge::Top => {
                    // Add extra_y to the cursor movement
                    self.cursor.y += scaled_h + gap + extra_y;
                    self.cursor.h = (self.cursor.h - scaled_h - gap - extra_y).max(0.0);
                }
                Edge::Bottom => {
                    // Subtract extra_y in height reduction
                    self.cursor.h = (self.cursor.h - scaled_h - gap - extra_y).max(0.0);
                }
            }
        }

        // Call the function with the new frame
        func(&mut Frame {
            cursor: child_rect_f32.shrink(margin),
            rect: child_rect_f32,
            margin: self.margin,
            gap: self.gap,
            scale: self.scale,
            fitting,
            _phantom: PhantomData,
        })
    }
}
