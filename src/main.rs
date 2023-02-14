
use windows::*;

use windows::Win32::Foundation::*;
use windows::Win32::System::LibraryLoader::GetModuleHandleA;
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::Win32::Graphics::Gdi::*;

use std::mem::MaybeUninit;

static mut BACKGROUND : MaybeUninit<(CreatedHDC, HBITMAP)> = MaybeUninit::uninit();

unsafe fn set_background(window : HWND, width : i32, height : i32) {

    let main = GetDC(window);
    let memdc = CreateCompatibleDC(main);
    let backing = CreateCompatibleBitmap(main, width, height);
    SelectObject(memdc, backing);

    for (x, y) in (1..100).into_iter().flat_map(|x| (1..100).into_iter().map(move |y| (x, y))) {
        //SetPixel(memdc, 10 + x, 10 + y, COLORREF(0x0));
    }
    BACKGROUND.write((memdc, backing));
}

fn main() {

    unsafe {
        let instance = GetModuleHandleA(None).unwrap();
        let window_class = s!("window");

        let wc = WNDCLASSA {
            hCursor: LoadCursorW(None, IDC_ARROW).unwrap(),
            hInstance: instance,
            lpszClassName: window_class,
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(callback),
            ..Default::default()
        };

        let atom = RegisterClassA(&wc);

        let handle = CreateWindowExA(
            WINDOW_EX_STYLE::default(),
            window_class,
            s!("window name"),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            1000,
            1000,
            None,
            None,
            instance,
            None
        );

        set_background(handle, 1000, 1000);

        let mut message = MSG::default();

        while GetMessageA(&mut message, HWND(0), 0, 0).into() {
            DispatchMessageA(&message);
        }
    }
    println!("Hello, world!");
}


extern "system" fn callback(window : HWND, message : u32, wparam : WPARAM, lparam : LPARAM) -> LRESULT {
    unsafe {
        match message {
            WM_SIZE => {
                let w = lparam.0 as u16 as f32; 
                let h = (lparam.0 >> 16) as f32;

                println!("w{} :: h{}", w, h);

                LRESULT(0)
            },
            WM_PAINT => {

                let z = GetDC(window);
                BitBlt(z, 0, 0, 1000, 1000, BACKGROUND.assume_init().0, 0, 0, SRCCOPY);

                LRESULT(0)
            }
            _ => DefWindowProcA(window, message, wparam, lparam),
        }
    }
}
