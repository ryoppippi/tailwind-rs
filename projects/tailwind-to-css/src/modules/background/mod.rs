use super::*;
mod attachment;
mod builder;
mod display;
#[cfg(test)]
mod test;

pub use self::attachment::TailwindBackgroundAttachment;

// Background Clip
// Utilities for controlling the bounding box of an element's background.
//
// ​
// Quick reference
// Class
// Properties
// bg-clip-border	background-clip: border-box;
// bg-clip-padding	background-clip: padding-box;
// bg-clip-content	background-clip: content-box;
// bg-clip-text	background-clip: text;
#[derive(Copy, Clone, Debug)]
enum BackgroundClip {}

// https://tailwindcss.com/docs/background-clip
#[derive(Copy, Clone, Debug)]
pub struct TailwindBackgroundClip {
    kind: BackgroundClip,
}

#[doc = include_str ! ("aspect-ratio.md")]
#[derive(Clone, Debug)]
pub struct TailwindBackgroundColor {
    pub(crate) color: ColorResolver,
}

// Background Origin
// Utilities for controlling how an element's background is positioned relative to borders, padding, and content.
//
// ​
// Quick reference
// Class
// Properties
// bg-origin-border	background-origin: border-box;
// bg-origin-padding	background-origin: padding-box;
// bg-origin-content	background-origin: content-box;
#[derive(Copy, Clone, Debug)]
enum BackgroundOrigin {}

// https://tailwindcss.com/docs/background-origin
#[derive(Copy, Clone, Debug)]
pub struct TailwindBackgroundOrigin {
    kind: BackgroundOrigin,
}

// https://tailwindcss.com/docs/background-origin
#[derive(Clone, Debug)]
pub struct TailwindBackgroundPosition {
    position: TailwindObjectPosition,
}

// Background Repeat
// Utilities for controlling the repetition of an element's background image.
//
// ​
// Quick reference
// Class
// Properties
// bg-repeat	background-repeat: repeat;
// bg-no-repeat	background-repeat: no-repeat;
// bg-repeat-x	background-repeat: repeat-x;
// bg-repeat-y	background-repeat: repeat-y;
// bg-repeat-round	background-repeat: round;
// bg-repeat-space	background-repeat: space;
#[derive(Copy, Clone, Debug)]
enum BackgroundRepeat {}

// https://tailwindcss.com/docs/background-origin
#[derive(Copy, Clone, Debug)]
pub struct TailwindBackgroundRepeat {
    kind: BackgroundOrigin,
}

#[doc = include_str ! ("aspect-ratio.md")]
#[derive(Clone, Debug)]
pub struct TailwindBackgroundBrightness {
    brightness: TailwindBrightness,
}
