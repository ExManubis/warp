// Shared spacing constants for context chips and related UDI elements.
// Centralizing these values keeps spacing consistent across chips and makes
// it easy to adjust the overall rhythm in one place.

/// Vertical padding inside of a chip
pub const UDI_CHIP_VERTICAL_PADDING: f32 = 2.0;
/// Horizontal padding inside of a chip
pub const UDI_CHIP_HORIZONTAL_PADDING: f32 = 4.0;
/// Space between icon and label inside a chip
pub const UDI_CHIP_ICON_GAP: f32 = 4.0;
/// Consistent margins surrounding all chips
pub const UDI_CHIP_MARGIN: f32 = 8.0;
/// Spacing between rows when chips wrap
pub const UDI_ROW_RUN_SPACING: f32 = 8.0;
/// Top padding factor for universal developer input prompt - less top padding
pub const UDI_PROMPT_TOP_PADDING_FACTOR: f32 = 0.6;
/// Bottom padding factor for universal developer input prompt - more bottom padding
pub const UDI_PROMPT_BOTTOM_PADDING_FACTOR: f32 = 1.5;
/// Bottom padding for classic prompt attach images
pub const CLASSIC_PROMPT_ATTACH_IMAGES_BOTTOM_PADDING: f32 = 10.;
/// Outer inset around the Warp prompt card
pub const WARP_PROMPT_CARD_MARGIN: f32 = 8.0;
/// Corner radius of the Warp prompt card
pub const WARP_PROMPT_CARD_RADIUS: f32 = 8.0;
/// Symmetric horizontal padding inside the Warp prompt card
pub const WARP_PROMPT_CARD_INNER_PADDING: f32 = 12.0;
/// Theme-background veil over the terminal (0–100). High enough to read as a
/// panel, low enough that scrollback shows through.
pub const WARP_PROMPT_CARD_VEIL_OPACITY: u8 = 55;
