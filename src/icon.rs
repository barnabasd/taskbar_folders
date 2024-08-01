use windows_sys::Win32::{Foundation::HANDLE, Graphics::Gdi::{GetObjectW, BITMAP}, UI::{Shell::SHDefExtractIconW, WindowsAndMessaging::{GetIconInfo, HICON, ICONINFO}}};
use crate::{action::Action, util::to_wstring};
use std::ptr::null_mut;

pub fn get(action: Action) -> HICON {
    if let Some(icon) = action.custom_icon {
        return extract_icon(&icon).0;
    } extract_icon(&action.task as &str).0
}

fn extract_icon(path: &str) -> (HICON, i32, i32) {
    unsafe {
        let mut hicon_large: HICON = null_mut();
        SHDefExtractIconW(
            to_wstring(path).as_ptr(), 0, 0, &mut hicon_large, &mut null_mut(), 64 | (64 << 16)
        );
        let mut icon_info: ICONINFO = std::mem::zeroed();
        GetIconInfo(hicon_large, &mut icon_info);
        let hbitmap = icon_info.hbmColor;
        let mut bitmap: BITMAP = std::mem::zeroed();
        GetObjectW(hbitmap as HANDLE, std::mem::size_of::<BITMAP>() as i32, &mut bitmap as *mut _ as *mut _ );
        return (hicon_large, bitmap.bmWidth, bitmap.bmHeight);
    }
}