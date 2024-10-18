use std::{
    ffi::OsStr,
    os::windows::ffi::OsStrExt,
    ptr,
    time::{SystemTime, UNIX_EPOCH},
};

use windows::{
    core::{w, HSTRING, PCWSTR},
    Win32::{
        Foundation::{HWND, LPARAM, LRESULT, WPARAM},
        System::LibraryLoader::GetModuleHandleW,
        UI::WindowsAndMessaging::{
            CreateWindowExW, DefWindowProcW, LoadCursorW,
            PostQuitMessage, RegisterClassExW, ShowWindow,
            CW_USEDEFAULT, IDC_ARROW, SW_SHOW, WINDOW_EX_STYLE, WM_DESTROY, WNDCLASSEXW, WS_OVERLAPPEDWINDOW, WS_VISIBLE,
        },
    },
};

pub struct Window {
    handle: HWND,
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

            ShowWindow(hwnd, SW_SHOW);

            Ok(Box::new(Window { handle: hwnd }))
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
        _ => {
            return DefWindowProcW(hwnd, msg, wparam, lparam);
        }
    }
}
