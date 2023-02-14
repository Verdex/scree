
use windows::*;

use windows::Win32::Foundation::*;
use windows::Win32::System::LibraryLoader::GetModuleHandleA;
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::Win32::Graphics::Gdi::*;

use std::mem::MaybeUninit;

static mut BACKGROUND : MaybeUninit<(CreatedHDC, HBITMAP)> = MaybeUninit::uninit();

unsafe fn init(x : HWND) {
    let z = GetDC(x);
    let memdc = CreateCompatibleDC(z);
    let backing = CreateCompatibleBitmap(z, 1000, 1000);
    SelectObject(memdc, backing);
    
    for (x, y) in (1..100).into_iter().flat_map(|x| (1..100).into_iter().map(move |y| (x, y))) {
        SetPixel(memdc, 10 + x, 10 + y, COLORREF(0xFFFFFFFF));
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

        init(handle);

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
            WM_PAINT => {

                let now = std::time::Instant::now();

                let z = GetDC(window);
                BitBlt(z, 0, 0, 1000, 1000, BACKGROUND.assume_init().0, 0, 0, SRCCOPY);

                println!("{}", now.elapsed().as_millis());

                LRESULT(0)
            }
            _ => DefWindowProcA(window, message, wparam, lparam),
        }
    }
}
