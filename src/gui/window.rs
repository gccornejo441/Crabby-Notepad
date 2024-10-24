use std::{
    ffi::{c_void, OsStr}, fmt::Error, fs::read_to_string, mem, os::windows::ffi::OsStrExt, ptr, thread::sleep, time::{SystemTime, UNIX_EPOCH}
};

use windows::{
    core::{w, HSTRING, PCWSTR},
    Win32::{
        Foundation::{COLORREF, HWND, LPARAM, LRESULT, RECT, WPARAM},
        Graphics::Gdi::{
            BeginPaint, CreateSolidBrush, DeleteObject, EndPaint, FillRect, InvalidateRect, TextOutW, PAINTSTRUCT
        },
        System::{
            Com::{
                CoCreateInstance, CoIncrementMTAUsage, CoInitializeEx, CoTaskMemFree, CLSCTX_ALL,
                COINIT_APARTMENTTHREADED,
            },
            LibraryLoader::GetModuleHandleW,
        },
        UI::{
            Shell::{
                Common::COMDLG_FILTERSPEC, FileOpenDialog, FileSaveDialog, IFileOpenDialog, IFileSaveDialog, OpenControlPanel, FOS_ALLNONSTORAGEITEMS, SIGDN_FILESYSPATH
            },
            WindowsAndMessaging::{
                AppendMenuW, CreateMenu, CreateWindowExW, DefWindowProcW, GetClientRect, GetWindowLongPtrW, GetWindowTextW, LoadCursorW, PostQuitMessage, RegisterClassExW, SetMenu, SetWindowLongPtrW, SetWindowTextW, ShowWindow, CREATESTRUCTW, CW_USEDEFAULT, ES_MULTILINE, GWLP_USERDATA, HMENU, IDC_ARROW, MF_POPUP, MF_STRING, SW_SHOW, WINDOW_EX_STYLE, WM_COMMAND, WM_CREATE, WM_DESTROY, WM_PAINT, WNDCLASSEXW, WS_BORDER, WS_CHILD, WS_OVERLAPPEDWINDOW, WS_VISIBLE, WS_VSCROLL
            },
        },
    },
};
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref FILE_CONTENT: Mutex<String> = Mutex::new(String::new());
}

pub struct Window {
    handle: HWND,
    edit_control: HWND,
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
            let mut window = Box::new(Window {
                handle: HWND(std::ptr::null_mut()),
                edit_control: HWND(std::ptr::null_mut()),
            });

            let window_ptr = Box::into_raw(window);

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
                Some(window_ptr as *const _ as *const c_void),
            )?;

            let edit_control = CreateWindowExW(
                WINDOW_EX_STYLE(0),
                w!("EDIT"),
                PCWSTR::null(),
                WS_CHILD | WS_VISIBLE | WS_BORDER | WS_VSCROLL,
                0,
                0,
                width as i32,
                height as i32,
                hwnd,
                None,
                hinstance,
                None,
            )?;

            Ok(Box::from_raw(window_ptr))
        }
    }
}
fn save_file(edit_control: HWND) -> Result<(), Box<dyn std::error::Error>> {
    let dialog: IFileSaveDialog = unsafe {
        CoCreateInstance(&FileSaveDialog, None, CLSCTX_ALL)?
    };

    unsafe {
        if dialog.Show(None).is_ok() {
            let result = dialog.GetResult()?;
            let path = result.GetDisplayName(SIGDN_FILESYSPATH)?;

            let mut buffer = vec![0u16; 1024];
            let len = GetWindowTextW(edit_control, &mut buffer) as usize;

            let content = String::from_utf16_lossy(&buffer[..len]);

            std::fs::write(PCWSTR(path.0).to_string()?, content)?;
        }
    }
    Ok(())
}

fn open_file(hwnd: HWND, edit_control: HWND) -> Result<(), Box<dyn std::error::Error>> {
    let dialog: IFileOpenDialog = unsafe {
        CoCreateInstance(&FileOpenDialog, None, CLSCTX_ALL)?
    };

    unsafe {
        let mut dialog_options = 0;
        dialog.GetOptions()?;
        dialog.SetOptions(FOS_ALLNONSTORAGEITEMS)?;
    }

    unsafe {
        if dialog.Show(None).is_ok() {
            let result = dialog.GetResults()?;
            let item = result.GetItemAt(0)?;
            let path = item.GetDisplayName(SIGDN_FILESYSPATH)?;

            let file_path = PCWSTR(path.0).to_string()?;
            let file_content = std::fs::read_to_string(file_path)?;

            SetWindowTextW(edit_control, &HSTRING::from(file_content));
        }
    }
    Ok(())
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
        WM_CREATE => {
            let create_struct = &*(lparam.0 as *mut CREATESTRUCTW);
            let window_ptr = (*create_struct).lpCreateParams as *mut Window;

            if !window_ptr.is_null() {
                SetWindowLongPtrW(hwnd, GWLP_USERDATA, window_ptr as isize);
                (*window_ptr).handle = hwnd;
            }
            LRESULT(0)
        }
        WM_COMMAND => {
            let window_ptr = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut Window;
    
            if !window_ptr.is_null() {
                let window = &*window_ptr;

                let menu_id = wparam.0 as u16;
                match menu_id {
                    1 => println!("File -> New clicked!"),
                    2 => {
                        if let Err(e) = open_file(window.handle, window.edit_control) {
                            eprintln!("Error opening file: {:?}", e);
                        }
                    }
                    3 => {
                        if let Err(e) = save_file(window.edit_control) {
                            eprintln!("Error saving file: {:?}", e);
                        }
                    }
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
            }
            println!("window_ptr is null");
            return LRESULT(0);
        }
        WM_PAINT => {
            let mut lppaint = PAINTSTRUCT::default();
            let hdc = BeginPaint(hwnd, &mut lppaint);

            let mut rect = RECT::default();
            GetClientRect(hwnd, &mut rect);

            let dark_brush = CreateSolidBrush(RGB(255, 255, 255));

            FillRect(hdc, &rect, dark_brush);
            DeleteObject(dark_brush);

            EndPaint(hwnd, &mut lppaint);

            LRESULT(0)
        }
        _ => DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}
