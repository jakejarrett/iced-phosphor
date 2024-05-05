//! The default icon font of the widgets of this library.

use cfg_if::cfg_if;
use iced::Font;

cfg_if! {
    if #[cfg(feature = "icons")] {
        pub mod phosphor;
        pub mod phosphor_light;

        pub use phosphor::Phosphor;
        pub use nerd::Nerd;
        /// The default icon font bytes for loading the font into iced.
        pub const BOOTSTRAP_FONT_BYTES: &[u8] = include_bytes!("./fonts/bootstrap-icons.ttf");
        /// the icon font that has all nerd fonts.
        pub const NERD_FONT_BYTES: &[u8] = include_bytes!("./fonts/nerd-icons.ttf");

        /// The bootstrap icon font.
        pub const BOOTSTRAP_FONT: Font = Font::with_name("bootstrap-icons");
        /// The nerd icon font.
        pub const NERD_FONT: Font = Font::with_name("Symbols Nerd Font");

        /// The default cupertino font bytes for loading the font into the system.
        pub const SF_UI_ROUNDED_BYTES: &[u8] = include_bytes!("./fonts/SFUIRounded.ttf");

        /// The default cupertino font for alerts and button.
        pub const SF_UI_ROUNDED: iced::Font = iced::Font::with_name(".SF UI Rounded");
    } else {
        pub mod required;
        // pub use required::{Bootstrap, icon_to_char, icon_to_string};
        /// The default icon font bytes for loading the font into iced.
        pub const BOOTSTRAP_FONT_BYTES: &[u8] = include_bytes!("./fonts/required-icons.ttf");
        /// The default icon font.
        pub const BOOTSTRAP_FONT: Font = Font::with_name("required-icons");
    }

}