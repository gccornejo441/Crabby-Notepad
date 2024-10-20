use std::{
    ffi::OsStr,
    os::windows::ffi::OsStrExt,
    ptr,
    time::{SystemTime, UNIX_EPOCH},
};

use windows::{
    core::{w, HSTRING, PCWSTR},
    Win32::{
        Foundation::{COLORREF, HWND, LPARAM, LRESULT, RECT, WPARAM}, Graphics::Gdi::{BeginPaint, CreateSolidBrush, DeleteObject, EndPaint, FillRect, HBRUSH, HGDIOBJ, PAINTSTRUCT}, System::LibraryLoader::GetModuleHandleW, UI::WindowsAndMessaging::{
            AppendMenuW, CreateMenu, CreateWindowExW, DefWindowProcW, GetClientRect, LoadCursorW, PostQuitMessage, RegisterClassExW, SetMenu, ShowWindow, CW_USEDEFAULT, HMENU, IDC_ARROW, MF_STRING, SW_SHOW, WINDOW_EX_STYLE, WM_DESTROY, WM_PAINT, WNDCLASSEXW, WS_OVERLAPPEDWINDOW, WS_VISIBLE
        }
    },
};

pub struct Window {
    handle: HWND,
    menu: HMENU,
}

fn RGB(r: u8, g: u8, b: u8) -> COLORREF {
    COLORREF((r as u32) | ((g as u32) << 8) | ((b as u32) << 16))
}

impl Window {
    pub fn new(title: &str, width: u32, height: u32) -> Result<Box<Self>, windows::core::Error> {
        let window_title = HSTRING::from(title);
        const WINDOW_CLASS: PCWSTR = w!("crabby-notepad");

        unsafe {
            let hinstance = GetModuleHandleW(None)?;

            let class = WNDCLASSEXW {
                cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
                hCursor: LoadCursorW(None, IDC_ARROW).ok().unwrap(),
                hInstance: hinstance.into(),
                lpszClassName: WINDOW_CLASS,
                lpfnWndProc: Some(wnd_proc),
                ..Default::default()
            };

            if RegisterClassExW(&class) == 0 {
                return Err(windows::core::Error::from_win32());
            }

            let hwnd = CreateWindowExW(
                WINDOW_EX_STYLE(0),
                WINDOW_CLASS,
                &window_title,
                WS_OVERLAPPEDWINDOW | WS_VISIBLE,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                width as i32,
                height as i32,
                None,
                None,
                hinstance,
                None,
            )?;

            let hmenu = CreateMenu();
            let file_menu = CreateMenu();
            let file_menu = hmenu.unwrap();
            AppendMenuW(file_menu, MF_STRING, 1, w!("File"));
            SetMenu(hwnd, file_menu);

            ShowWindow(hwnd, SW_SHOW);

            Ok(Box::new(Window { handle: hwnd, menu: file_menu }))
        }
    }
}

unsafe extern "system" fn wnd_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        WM_DESTROY => {
            PostQuitMessage(0);
            return LRESULT(0);
        }
        WM_PAINT => {
            let mut lppaint = PAINTSTRUCT::default();
            let hdc = BeginPaint(hwnd, &mut lppaint);
            let mut rect: RECT = RECT {
                left: 0,
                top: 0,
                right: 0,
                bottom: 0,
            };
            GetClientRect(hwnd, &mut rect);
            
            let sandish_color = RGB(240, 238, 229);
            let white_color = RGB(255,255,255);
            let white_brush: HBRUSH = CreateSolidBrush(white_color);

            FillRect(hdc, &rect, white_brush);
        
            EndPaint(hwnd, &lppaint);
        
            return LRESULT(0);
        }        
        _ => {
            return DefWindowProcW(hwnd, msg, wparam, lparam);
        }
    }
}
