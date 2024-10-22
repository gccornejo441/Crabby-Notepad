use std::{
    ffi::OsStr,
    mem,
    os::windows::ffi::OsStrExt,
    ptr,
    time::{SystemTime, UNIX_EPOCH},
};

use windows::{
    core::{w, HSTRING, PCWSTR},
    Win32::{
        Foundation::{COLORREF, HWND, LPARAM, LRESULT, RECT, WPARAM},
        Graphics::Gdi::{
            BeginPaint, CreateFontW, CreateSolidBrush, DeleteObject, DrawTextW, EndPaint, FillRect,
            GetWindowDC, LineTo, MoveToEx, Rectangle, ReleaseDC, SelectObject, SetBkColor,
            SetBkMode, SetTextColor, TextOutW, CHARSET_UNICODE, CLIP_DEFAULT_PRECIS,
            DEFAULT_QUALITY, DT_CENTER, DT_SINGLELINE, FF_DONTCARE, FW_DONTCARE, HBRUSH, HFONT,
            HGDIOBJ, OUT_TT_PRECIS, PAINTSTRUCT, TRANSPARENT,
        },
        System::LibraryLoader::GetModuleHandleW,
        UI::{
            Controls::{DRAWITEMSTRUCT, MEASUREITEMSTRUCT},
            WindowsAndMessaging::{
                AppendMenuW, CreateMenu, CreateWindowExW, DefWindowProcW, GetClientRect,
                GetWindowRect, LoadCursorW, PostQuitMessage, RegisterClassExW, SetMenu,
                SetMenuInfo, ShowWindow, CW_USEDEFAULT, HMENU, IDC_ARROW, MENUINFO, MENUINFO_MASK,
                MENUINFO_STYLE, MF_CHECKED, MF_OWNERDRAW, MF_POPUP, MF_STRING, MIM_APPLYTOSUBMENUS,
                MIM_STYLE, MNS_NOTIFYBYPOS, SW_SHOW, WINDOW_EX_STYLE, WM_COMMAND, WM_DESTROY,
                WM_DRAWITEM, WM_MEASUREITEM, WM_NCPAINT, WM_PAINT, WNDCLASSEXW,
                WS_OVERLAPPEDWINDOW, WS_VISIBLE,
            },
        },
    },
};

pub struct Window {
    handle: HWND,
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

            let hmenu = create_menus()?;
            SetMenu(hwnd, hmenu);

            ShowWindow(hwnd, SW_SHOW);

            Ok(Box::new(Window { handle: hwnd }))
        }
    }
}

unsafe fn create_menus() -> Result<HMENU, windows::core::Error> {
    let hmenu = CreateMenu()?;
    if hmenu.is_invalid() {
        return Err(windows::core::Error::from_win32());
    }

    let file_menu = CreateMenu()?;
    let edit_menu = CreateMenu()?;
    let view_menu = CreateMenu()?;

    AppendMenuW(file_menu, MF_STRING, 1, w!("New"));
    AppendMenuW(file_menu, MF_STRING, 2, w!("Open"));
    AppendMenuW(file_menu, MF_STRING, 3, w!("Save"));

    AppendMenuW(edit_menu, MF_STRING, 4, w!("Undo"));
    AppendMenuW(edit_menu, MF_STRING, 5, w!("Redo"));
    AppendMenuW(edit_menu, MF_STRING, 6, w!("Cut"));
    AppendMenuW(edit_menu, MF_STRING, 7, w!("Copy"));
    AppendMenuW(edit_menu, MF_STRING, 8, w!("Paste"));

    AppendMenuW(view_menu, MF_STRING, 9, w!("Zoom In"));
    AppendMenuW(view_menu, MF_STRING, 10, w!("Zoom Out"));
    AppendMenuW(view_menu, MF_STRING, 11, w!("Actual Size"));

    AppendMenuW(hmenu, MF_POPUP, file_menu.0 as usize, w!("File"));
    AppendMenuW(hmenu, MF_POPUP, edit_menu.0 as usize, w!("Edit"));
    AppendMenuW(hmenu, MF_POPUP, view_menu.0 as usize, w!("View"));

    Ok(hmenu)
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
        WM_COMMAND => {
            let menu_id = wparam.0 as u16;

            match menu_id {
                1 => println!("File -> New clicked!"),
                2 => println!("File -> Open clicked!"),
                3 => println!("File -> Save clicked!"),
                4 => println!("Edit -> Undo clicked!"),
                5 => println!("Edit -> Redo clicked!"),
                6 => println!("Edit -> Cut clicked!"),
                7 => println!("Edit -> Copy clicked!"),
                8 => println!("Edit -> Paste clicked!"),
                9 => println!("View -> Zoom In clicked!"),
                10 => println!("View -> Zoom Out clicked!"),
                11 => println!("View -> Actual Size clicked!"),
                _ => (),
            }
            return LRESULT(0);
        }
        WM_PAINT => {
            let mut lppaint = PAINTSTRUCT::default();
            let hdc = BeginPaint(hwnd, &mut lppaint);
            
            let mut rect = RECT::default();
            GetClientRect(hwnd, &mut rect);

            let dark_brush = CreateSolidBrush(RGB(255,255, 255));

            FillRect(hdc, &rect, dark_brush);
            DeleteObject(dark_brush);

            EndPaint(hwnd, &mut lppaint);

            LRESULT(0)
        }
        _ => DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}