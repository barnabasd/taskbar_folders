use winreg::RegKey;

const SUBKEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Themes\\Personalize";
const VALUE: &str = "AppsUseLightTheme";

pub fn is_dark_theme() -> bool {
    let hkcu = RegKey::predef(winreg::enums::HKEY_CURRENT_USER);
    if let Ok(subkey) = hkcu.open_subkey(SUBKEY) {
        if let Ok(dword) = subkey.get_value::<u32, _>(VALUE) {
            return dword == 0;
        }
    }
    false
}

pub enum Color {
    Border, Background, IconHover
}

pub fn get_val_from_theme(color: Color) -> u32 {
    let theme_is_dark = is_dark_theme();
    match color {
        Color::Background => return if theme_is_dark { 0xFF1c1c1c } else { 0xFFEEEEEE },
        Color::IconHover => return if theme_is_dark { 0xFF292929 } else { 0xFFF7F7F7 },
        Color::Border => return if theme_is_dark { 0xFF404040 } else { 0xFFBEBEBE },
    }
}