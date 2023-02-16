
use windows::*;

use windows::Win32::Foundation::*;
use windows::Win32::System::LibraryLoader::GetModuleHandleA;
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::Win32::Graphics::Gdi::*;

use std::mem::MaybeUninit;

static mut BACKGROUND : MaybeUninit<(CreatedHDC, HBITMAP)> = MaybeUninit::uninit();

// TODO:  SelectObject has a dual in DestroyObject
unsafe fn set_background(width : i32, height : i32) {

    /*let main = GetDC(HWND::default());
    let memdc = CreateCompatibleDC(main);
    let backing = CreateCompatibleBitmap(main, width, height);
    SelectObject(memdc, backing);

    for (x, y) in (0..width).into_iter().flat_map(|x| (0..height).into_iter().map(move |y| (x, y))) {
        SetPixel(memdc, x, y, COLORREF(0x00FF00FF)); // NOTE:  00BBGGRR
    }
    BACKGROUND.write((memdc, backing));*/
}

fn main() {

    unsafe {
        let instance = GetModuleHandleA(None).unwrap();
        let window_class = s!("window");

        let background_brush = CreateSolidBrush(COLORREF(0x00FFFF00));
        let wc = WNDCLASSA {
            hCursor: LoadCursorW(None, IDC_ARROW).unwrap(),
            hInstance: instance,
            lpszClassName: window_class,
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(callback),
            hbrBackground: background_brush, 
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

        //set_background(1000, 1000);

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
                let w = lparam.0 as u16 as i32; 
                let h = (lparam.0 >> 16) as i32;

                println!("w{} :: h{}", w, h);

                //set_background(w, h);
                

                LRESULT(0)
            },
            WM_PAINT => {
                let z = GetDC(window);

                let mut a = POINT::default();
                a.x = 300;
                a.y = 300;
                let mut b = POINT::default();
                b.x = 400;
                b.y = 400;
                let c = [a, b];
                SetDCPenColor(z, COLORREF(0x00000000));
                Polyline(z, &c);

                let mut a = POINT::default();
                a.x = 0;
                a.y = 0;
                let mut b = POINT::default();
                b.x = 100;
                b.y = 100;
                let mut c = POINT::default();
                c.x = 600;
                c.y = 600;
                let mut d = POINT::default();
                d.x = 700;
                d.y = 700;
                let c = [a, b, c, d];
                let w = [2, 2];
                PolyPolyline(z, &c as *const _, &w);

                //BitBlt(z, 0, 0, 1000, 1000, BACKGROUND.assume_init().0, 0, 0, SRCCOPY);

                LRESULT(0)
            }
            /*WM_QUIT => {

            }, */
            _ => DefWindowProcA(window, message, wparam, lparam),
        }
    }
}
