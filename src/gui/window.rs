use std::{
    ffi::OsStr, mem, os::windows::ffi::OsStrExt, ptr, time::{SystemTime, UNIX_EPOCH}
};

use windows::{
    core::{w, HSTRING, PCWSTR},
    Win32::{
        Foundation::{COLORREF, HWND, LPARAM, LRESULT, RECT, WPARAM},
        Graphics::Gdi::{
            BeginPaint, CreateSolidBrush, DeleteObject, DrawTextW, EndPaint, FillRect, GetWindowDC, ReleaseDC, SetBkColor, SetTextColor, DT_CENTER, DT_SINGLELINE, HBRUSH, HGDIOBJ, PAINTSTRUCT
        },
        System::LibraryLoader::GetModuleHandleW,
        UI::{
            Controls::{DRAWITEMSTRUCT, MEASUREITEMSTRUCT},
            WindowsAndMessaging::{
                AppendMenuW, CreateMenu, CreateWindowExW, DefWindowProcW, GetClientRect, LoadCursorW, PostQuitMessage, RegisterClassExW, SetMenu, SetMenuInfo, ShowWindow, CW_USEDEFAULT, HMENU, IDC_ARROW, MENUINFO, MENUINFO_MASK, MENUINFO_STYLE, MF_CHECKED, MF_OWNERDRAW, MF_POPUP, MF_STRING, MIM_APPLYTOSUBMENUS, MIM_STYLE, MNS_NOTIFYBYPOS, SW_SHOW, WINDOW_EX_STYLE, WM_COMMAND, WM_DESTROY, WM_DRAWITEM, WM_MEASUREITEM, WM_NCPAINT, WM_PAINT, WNDCLASSEXW, WS_OVERLAPPEDWINDOW, WS_VISIBLE
            },
        },
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

            let menu = CreateMenu()?;

            AppendMenuW(menu, MF_OWNERDRAW, 100, w!("New"));
            AppendMenuW(menu, MF_OWNERDRAW, 101, w!("Open"));
            SetMenu(hwnd, menu);

            ShowWindow(hwnd, SW_SHOW);

            Ok(Box::new(Window {
                handle: hwnd,
                menu: menu,
            }))
        }
    }
}

unsafe fn draw_menu_item(draw_item_struct: DRAWITEMSTRUCT) {
    let mut dis = draw_item_struct;
    let hdc = dis.hDC;

    SetBkColor(hdc, RGB(0, 128, 128));
    SetTextColor(hdc, RGB(255, 255, 255));

    let brush = CreateSolidBrush(RGB(0, 128, 128));
    FillRect(hdc, &dis.rcItem, brush);
    DeleteObject(brush);

    let text = match dis.itemID {
        100 => "New",
        101 => "Open",
        _ => "",
    };

    let mut text_wide: Vec<u16> = text.encode_utf16().collect();
    let mut rect = dis.rcItem;
    DrawTextW(hdc, &mut text_wide, &mut rect, DT_CENTER | DT_SINGLELINE);
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

            let mut rect = RECT::default();
            GetClientRect(hwnd, &mut rect); 
            FillRect(hdc, &rect, CreateSolidBrush(RGB(255,0,0)));
            EndPaint(hwnd, &lppaint);
            return LRESULT(0);
        }
        WM_NCPAINT => {
            let hdc = GetWindowDC(hwnd);
            
            let mut rect = RECT::default();
            GetClientRect(hwnd, &mut rect);

            let brush = CreateSolidBrush(RGB(0, 128, 128)); 
            FillRect(hdc, &rect, brush);
            DeleteObject(brush);
            
            ReleaseDC(hwnd, hdc);
            return LRESULT(0);
        }
        WM_DRAWITEM => {
            let dis = &*(lparam.0 as *const DRAWITEMSTRUCT);
            draw_menu_item(*dis);
            return LRESULT(0);
        }
        WM_MEASUREITEM => {
            let mis = &mut *(lparam.0 as *mut MEASUREITEMSTRUCT);
            mis.itemWidth = 100;
            mis.itemHeight = 30;
            return LRESULT(0);
        }
        WM_COMMAND => {
            let menu_id = wparam.0 as u16;

            match menu_id {
                1 => {
                    println!("File menu clicked!");
                }
                _ => (),
            }
            return LRESULT(0);
        }
        _ => {
            return DefWindowProcW(hwnd, msg, wparam, lparam);
        }
    }
}