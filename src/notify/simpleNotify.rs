use std::{fmt::Error, os::windows};

use ::windows::{core::s, Win32::{Foundation::{HWND, LPARAM, LRESULT, WPARAM}, Graphics::Gdi::ValidateRect, System::LibraryLoader::GetModuleHandleA, UI::WindowsAndMessaging::{CreateWindowExA, DefWindowProcA, DispatchMessageA, GetMessageA, LoadCursorW, PostQuitMessage, RegisterClassA, CS_HREDRAW, CS_VREDRAW, CW_USEDEFAULT, IDC_ARROW, MSG, WINDOW_EX_STYLE, WNDCLASSA, WS_OVERLAPPEDWINDOW, WS_VISIBLE}}};


pub fn simpleNotify()->Result<(), Box<dyn std::error::Error>>{
    unsafe {
        let instance = GetModuleHandleA(None)?;
        debug_assert!(instance.0 != std::ptr::null_mut());

        let window_class = s!("window");

        let wc = WNDCLASSA {
            hCursor: LoadCursorW(None, IDC_ARROW)?,
            hInstance: instance.into(),
            lpszClassName: window_class,

            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(wndproc),
            ..Default::default()
        };

        let atom = RegisterClassA(&wc);
        debug_assert!(atom != 0);

        CreateWindowExA(
            WINDOW_EX_STYLE::default(),
            window_class,
            s!("This is a sample window"),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            None,
            None,
            instance,
            None,
        );

        let mut message = MSG::default();

        while GetMessageA(&mut message, HWND(std::ptr::null_mut()), 0, 0).into() {
            DispatchMessageA(&message);
        }

        Ok(())
    }
}

extern "system" fn wndproc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        match message {
            WM_PAINT => {
                println!("WM_PAINT");
                ValidateRect(window, None);
                LRESULT(0)
            }
            WM_DESTROY => {
                println!("WM_DESTROY");
                PostQuitMessage(0);
                LRESULT(0)
            }
            _ => DefWindowProcA(window, message, wparam, lparam),
        }
    }
}