use windows_sys::Win32::Graphics::GdiPlus::{ GdipCreateBitmapFromHICON, GdipCreateFromHDC, GdipCreateSolidFill, GdipDeleteGraphics, GdipDrawImageRect, GdipFillPie, GdipFillRectangle, GdipSetPixelOffsetMode, GdipSetSmoothingMode, GpBitmap, GpBrush, GpGraphics, GpImage, GpSolidFill, PixelOffsetModeHighQuality, SmoothingModeAntiAlias };
use windows_sys::Win32::Graphics::Gdi::{ BeginPaint, BitBlt, CreateCompatibleBitmap, CreateCompatibleDC, DeleteDC, DeleteObject, EndPaint, SelectObject, PAINTSTRUCT, SRCCOPY };
use windows_sys::Win32::Foundation::{ HWND, LPARAM, RECT, WPARAM };
use crate::window::{InternalIconState, State};
use std::ptr::null_mut;
use crate::icon::get;

const _TASBAR_BORDER_LIGHT: u32 = 0xFF000000;
const _TASKBAR_BG_LIGHT: u32 = 0xFFFFFFFF;
const _ICON_HOVER_LIGHT: u32 = 0xFFFFFFFF;

const TASBAR_BORDER_DARK: u32 = 0xFF404040;
const TASKBAR_BG_DARK: u32 = 0xFF1c1c1c;
const ICON_HOVER_DARK: u32 = 0xFF292929;

pub unsafe fn wm_paint(hwnd: HWND, _wp: WPARAM, _lp: LPARAM, state: &State, internal_state: InternalIconState) {
    let mut ps: PAINTSTRUCT = std::mem::zeroed();
    let hdc_screen = BeginPaint(hwnd, &mut ps);
    let hdc_mem = CreateCompatibleDC(hdc_screen);
    let hbm_mem = CreateCompatibleBitmap(hdc_screen, 
        (ps.rcPaint.right - ps.rcPaint.left) as i32, 
        (ps.rcPaint.bottom - ps.rcPaint.top) as i32);
    let hbm_old = SelectObject(hdc_mem, hbm_mem);
    let mut graphics: *mut GpGraphics = null_mut();
    GdipCreateFromHDC(hdc_mem, &mut graphics);
    GdipSetSmoothingMode(graphics, SmoothingModeAntiAlias);
    GdipSetPixelOffsetMode(graphics, PixelOffsetModeHighQuality);
    let mut brush: *mut GpSolidFill = null_mut();
    GdipCreateSolidFill(TASBAR_BORDER_DARK, &mut brush);
    GdipFillRoundedRect(graphics, brush as *mut GpBrush, 0, 0, 
        ps.rcPaint.right - ps.rcPaint.left,
        ps.rcPaint.bottom - ps.rcPaint.top, 8);
    GdipCreateSolidFill(TASKBAR_BG_DARK, &mut brush);
    GdipFillRoundedRect(graphics, brush as *mut GpBrush, 1, 1,
        ps.rcPaint.right - ps.rcPaint.left - 2,
        ps.rcPaint.bottom - ps.rcPaint.top - 2, 8);
    for (i, icon) in state.iter().enumerate() {
        let bounds: RECT = RECT {left: (3 + (i * 44)) as i32, bottom: 43, top: 3, right: (43 + (i * 44)) as i32 };
        let mut brush: *mut GpSolidFill = null_mut();
        let color = if (internal_state.icons[i]).0 { ICON_HOVER_DARK } else { TASKBAR_BG_DARK };
        GdipCreateSolidFill(color, &mut brush);
        GdipFillRoundedRect(graphics, brush as *mut GpBrush, bounds.left, bounds.top, bounds.right - bounds.left, bounds.bottom - bounds.top, 8);
        let mut gp_bitmap: *mut GpBitmap = null_mut();
        GdipCreateBitmapFromHICON(get(icon.clone()), &mut gp_bitmap);
        let size = if (internal_state.icons[i]).1 { 16.0 } else { 24.0 };
        let top = if (internal_state.icons[i]).1 { bounds.top + 12 } else { bounds.top + 8 };
        let left = if (internal_state.icons[i]).1 { bounds.left + 12 } else { bounds.left + 8 };
        GdipDrawImageRect(graphics, gp_bitmap as *mut GpImage, left as f32, top as f32, size, size);
    }
    BitBlt(hdc_screen, ps.rcPaint.left, ps.rcPaint.top,
        (ps.rcPaint.right - ps.rcPaint.left) as i32,
        (ps.rcPaint.bottom - ps.rcPaint.top) as i32,
        hdc_mem, ps.rcPaint.left, ps.rcPaint.top, SRCCOPY);
    GdipDeleteGraphics(graphics);
    DeleteObject(SelectObject(hdc_mem, hbm_old));
    DeleteDC(hdc_mem);
    EndPaint(hwnd, &ps);
}

#[allow(non_snake_case)]
unsafe fn GdipFillRoundedRect(graphics: *mut GpGraphics, brush: *mut GpBrush, x: i32, y: i32, w: i32, h: i32, rad: i32) {
    GdipFillPie(graphics, brush, x as f32, y as f32, (rad*2) as f32, (rad*2) as f32, 180., 90.);
    GdipFillPie(graphics, brush, ((x + w) - (rad*2)) as f32, y as f32, (rad*2) as f32, (rad*2) as f32, 270., 90.);
    GdipFillPie(graphics, brush, x as f32, ((y + h) - (rad*2)) as f32, (rad*2) as f32, (rad*2) as f32, 090., 90.);
    GdipFillPie(graphics, brush, ((x + w) - (rad*2)) as f32, ((y + h) - (rad*2)) as f32, (rad*2) as f32, (rad*2) as f32, 000., 90.);
    GdipFillRectangle(graphics, brush, (x + 8) as f32, y as f32, (w - (rad*2)) as f32, h as f32);
    GdipFillRectangle(graphics, brush, x as f32, (y + 8) as f32, w as f32, (h - (rad*2)) as f32);
}