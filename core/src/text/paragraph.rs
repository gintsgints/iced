//! Draw paragraphs.
use crate::alignment;
use crate::text::{Difference, Hit, Span, Text};
use crate::{Point, Size};

/// A text paragraph.
pub trait Paragraph: Sized + Default {
    /// The font of this [`Paragraph`].
    type Font: Copy + PartialEq;

    /// Creates a new [`Paragraph`] laid out with the given [`Text`].
    fn with_text(text: Text<&str, Self::Font>) -> Self;

    /// Creates a new [`Paragraph`] laid out with the given [`Text`].
    fn with_spans(text: Text<&[Span<'_, Self::Font>], Self::Font>) -> Self;

    /// Lays out the [`Paragraph`] with some new boundaries.
    fn resize(&mut self, new_bounds: Size);

    /// Compares the [`Paragraph`] with some desired [`Text`] and returns the
    /// [`Difference`].
    fn compare(&self, text: Text<(), Self::Font>) -> Difference;

    /// Returns the horizontal alignment of the [`Paragraph`].
    fn horizontal_alignment(&self) -> alignment::Horizontal;

    /// Returns the vertical alignment of the [`Paragraph`].
    fn vertical_alignment(&self) -> alignment::Vertical;

    /// Returns the minimum boundaries that can fit the contents of the
    /// [`Paragraph`].
    fn min_bounds(&self) -> Size;

    /// Tests whether the provided point is within the boundaries of the
    /// [`Paragraph`], returning information about the nearest character.
    fn hit_test(&self, point: Point) -> Option<Hit>;

    /// Returns the distance to the given grapheme index in the [`Paragraph`].
    fn grapheme_position(&self, line: usize, index: usize) -> Option<Point>;

    /// Returns the minimum width that can fit the contents of the [`Paragraph`].
    fn min_width(&self) -> f32 {
        self.min_bounds().width
    }

    /// Returns the minimum height that can fit the contents of the [`Paragraph`].
    fn min_height(&self) -> f32 {
        self.min_bounds().height
    }
}

/// A [`Paragraph`] of plain text.
#[derive(Debug, Clone, Default)]
pub struct Plain<P: Paragraph> {
    raw: P,
    content: String,
}

impl<P: Paragraph> Plain<P> {
    /// Creates a new [`Plain`] paragraph.
    pub fn new(text: Text<&str, P::Font>) -> Self {
        let content = text.content.to_owned();

        Self {
            raw: P::with_text(text),
            content,
        }
    }

    /// Updates the plain [`Paragraph`] to match the given [`Text`], if needed.
    pub fn update(&mut self, text: Text<&str, P::Font>) {
        if self.content != text.content {
            text.content.clone_into(&mut self.content);
            self.raw = P::with_text(text);
            return;
        }

        match self.raw.compare(Text {
            content: (),
            bounds: text.bounds,
            size: text.size,
            line_height: text.line_height,
            font: text.font,
            horizontal_alignment: text.horizontal_alignment,
            vertical_alignment: text.vertical_alignment,
            shaping: text.shaping,
        }) {
            Difference::None => {}
            Difference::Bounds => {
                self.raw.resize(text.bounds);
            }
            Difference::Shape => {
                self.raw = P::with_text(text);
            }
        }
    }

    /// Returns the horizontal alignment of the [`Paragraph`].
    pub fn horizontal_alignment(&self) -> alignment::Horizontal {
        self.raw.horizontal_alignment()
    }

    /// Returns the vertical alignment of the [`Paragraph`].
    pub fn vertical_alignment(&self) -> alignment::Vertical {
        self.raw.vertical_alignment()
    }

    /// Returns the minimum boundaries that can fit the contents of the
    /// [`Paragraph`].
    pub fn min_bounds(&self) -> Size {
        self.raw.min_bounds()
    }

    /// Returns the minimum width that can fit the contents of the
    /// [`Paragraph`].
    pub fn min_width(&self) -> f32 {
        self.raw.min_width()
    }

    /// Returns the cached [`Paragraph`].
    pub fn raw(&self) -> &P {
        &self.raw
    }
}
