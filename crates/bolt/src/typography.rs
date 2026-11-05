use gpui::App;
use std::borrow::Cow;

pub struct BundledFontAsset {
    pub family: &'static str,
    pub file_name: &'static str,
    pub data: &'static [u8],
}

// Metrics sweep checklist (design-bundle-application S9b.5):
// - Button label vertical centering at Heights::BUTTON (30px)
// - Input baseline at Heights::INPUT (30px)
// - Sidebar group label uppercase tracking
// - Doc tab label truncation at Heights::TAB (36px)
// - Status-bar items at Heights::TOOLBAR (32px)
// - Modal body min-height 96px
// - Table row text at Heights::ROW (28px) / ROW_COMPACT (24px)
pub struct AppFonts;

impl AppFonts {
    /// Interface face: labels, buttons, menus, settings, body copy, tabs and tree rows.
    /// Inter is the face JetBrains IDEs use for their interface.
    pub const INTERFACE: &'static str = "Inter";
    /// Section labels and large titles share the interface face, in sentence case,
    /// as IDE tool windows do.
    pub const DISPLAY: &'static str = "Inter";
    /// Data face: grid cells, keys, queries, identifiers, key hints and numeric readouts.
    pub const MONO: &'static str = "JetBrains Mono";
    pub const MONO_FALLBACK: &'static str = "monospace";
}

pub const BUNDLED_FONT_ASSETS: [BundledFontAsset; 12] = [
    BundledFontAsset {
        family: AppFonts::INTERFACE,
        file_name: "Inter-Regular.ttf",
        data: include_bytes!("../assets/fonts/Inter-Regular.ttf"),
    },
    BundledFontAsset {
        family: AppFonts::INTERFACE,
        file_name: "Inter-Medium.ttf",
        data: include_bytes!("../assets/fonts/Inter-Medium.ttf"),
    },
    BundledFontAsset {
        family: AppFonts::INTERFACE,
        file_name: "Inter-SemiBold.ttf",
        data: include_bytes!("../assets/fonts/Inter-SemiBold.ttf"),
    },
    BundledFontAsset {
        family: AppFonts::INTERFACE,
        file_name: "Inter-Bold.ttf",
        data: include_bytes!("../assets/fonts/Inter-Bold.ttf"),
    },
    BundledFontAsset {
        family: AppFonts::MONO,
        file_name: "JetBrainsMono-Regular.ttf",
        data: include_bytes!("../assets/fonts/JetBrainsMono-Regular.ttf"),
    },
    BundledFontAsset {
        family: AppFonts::MONO,
        file_name: "JetBrainsMono-Italic.ttf",
        data: include_bytes!("../assets/fonts/JetBrainsMono-Italic.ttf"),
    },
    BundledFontAsset {
        family: AppFonts::MONO,
        file_name: "JetBrainsMono-Medium.ttf",
        data: include_bytes!("../assets/fonts/JetBrainsMono-Medium.ttf"),
    },
    BundledFontAsset {
        family: AppFonts::MONO,
        file_name: "JetBrainsMono-MediumItalic.ttf",
        data: include_bytes!("../assets/fonts/JetBrainsMono-MediumItalic.ttf"),
    },
    BundledFontAsset {
        family: AppFonts::MONO,
        file_name: "JetBrainsMono-SemiBold.ttf",
        data: include_bytes!("../assets/fonts/JetBrainsMono-SemiBold.ttf"),
    },
    BundledFontAsset {
        family: AppFonts::MONO,
        file_name: "JetBrainsMono-SemiBoldItalic.ttf",
        data: include_bytes!("../assets/fonts/JetBrainsMono-SemiBoldItalic.ttf"),
    },
    BundledFontAsset {
        family: AppFonts::MONO,
        file_name: "JetBrainsMono-Bold.ttf",
        data: include_bytes!("../assets/fonts/JetBrainsMono-Bold.ttf"),
    },
    BundledFontAsset {
        family: AppFonts::MONO,
        file_name: "JetBrainsMono-BoldItalic.ttf",
        data: include_bytes!("../assets/fonts/JetBrainsMono-BoldItalic.ttf"),
    },
];

pub fn bundled_font_data() -> Vec<Cow<'static, [u8]>> {
    BUNDLED_FONT_ASSETS
        .iter()
        .map(|font| Cow::Borrowed(font.data))
        .collect()
}

/// Load all bundled fonts into GPUI's text system.
///
/// Called once during UI theme initialization. If registration fails, keep
/// the app running so GPUI can fall back to system fonts instead of aborting
/// startup.
pub fn load_bundled_fonts(cx: &mut App) {
    if let Err(error) = cx.text_system().add_fonts(bundled_font_data()) {
        eprintln!("failed to register bundled UI fonts, falling back to system fonts: {error}");
    }
}

#[cfg(test)]
mod tests {
    use super::{AppFonts, BUNDLED_FONT_ASSETS};

    #[test]
    fn font_contract_splits_interface_display_and_data_families() {
        assert_eq!(AppFonts::INTERFACE, "Inter");
        assert_eq!(AppFonts::DISPLAY, AppFonts::INTERFACE);
        assert_eq!(AppFonts::MONO, "JetBrains Mono");
        assert_eq!(AppFonts::MONO_FALLBACK, "monospace");
    }

    #[test]
    fn bundled_assets_cover_every_family() {
        let assets: Vec<_> = BUNDLED_FONT_ASSETS
            .iter()
            .map(|asset| (asset.family, asset.file_name))
            .collect();

        assert_eq!(
            assets,
            vec![
                (AppFonts::INTERFACE, "Inter-Regular.ttf"),
                (AppFonts::INTERFACE, "Inter-Medium.ttf"),
                (AppFonts::INTERFACE, "Inter-SemiBold.ttf"),
                (AppFonts::INTERFACE, "Inter-Bold.ttf"),
                (AppFonts::MONO, "JetBrainsMono-Regular.ttf"),
                (AppFonts::MONO, "JetBrainsMono-Italic.ttf"),
                (AppFonts::MONO, "JetBrainsMono-Medium.ttf"),
                (AppFonts::MONO, "JetBrainsMono-MediumItalic.ttf"),
                (AppFonts::MONO, "JetBrainsMono-SemiBold.ttf"),
                (AppFonts::MONO, "JetBrainsMono-SemiBoldItalic.ttf"),
                (AppFonts::MONO, "JetBrainsMono-Bold.ttf"),
                (AppFonts::MONO, "JetBrainsMono-BoldItalic.ttf"),
            ]
        );
    }
}
