use std::env;
use std::process::Command;
use std::time::{Duration, Instant};
use std::thread;
use std::sync::atomic::{AtomicI32, Ordering};

use windows::core::PCWSTR;
use windows::Win32::Foundation::*;
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::Win32::Graphics::Gdi::*;
use windows::Win32::System::Threading::*;
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use std::mem::zeroed;

// Global counter for animation
static FRAME_COUNTER: AtomicI32 = AtomicI32::new(0);

// Animation window class
struct SplashWindow {
    hwnd: HWND,
}

impl SplashWindow {
    unsafe fn create() -> Result<Self, String> {
        // Register the window class
        let class_name = "SplashWindowClass\0".encode_utf16().collect::<Vec<u16>>();
        
        let wc = WNDCLASSEXW {
            cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(Self::wnd_proc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: GetModuleHandleW(None).ok().unwrap(),
            hIcon: HICON(0),
            hCursor: LoadCursorW(None, IDC_ARROW).ok().unwrap(),
            hbrBackground: HBRUSH((COLOR_WINDOW.0 + 1) as isize),
            lpszMenuName: PCWSTR(std::ptr::null_mut()),
            lpszClassName: PCWSTR(class_name.as_ptr()),
            hIconSm: HICON(0),
        };
        
        RegisterClassExW(&wc);
        
        // Get the screen dimensions
        let screen_width = GetSystemMetrics(SM_CXSCREEN);
        let screen_height = GetSystemMetrics(SM_CYSCREEN);
        
        // Create the window
        let hwnd = CreateWindowExW(
            WS_EX_TOPMOST | WS_EX_LAYERED,
            PCWSTR(class_name.as_ptr()),
            PCWSTR("Loading...\0".encode_utf16().collect::<Vec<u16>>().as_ptr()),
            WS_POPUP | WS_VISIBLE,
            0, 0, screen_width, screen_height,
            None,
            None,
            GetModuleHandleW(None).ok().unwrap(),
            Some(std::ptr::null_mut()),
        );
        
        if hwnd.0 == 0 {
            return Err("Failed to create window".to_string());
        }
        
        // Make window black background
        SetLayeredWindowAttributes(hwnd, COLORREF(0), 255, LWA_ALPHA);
        
        // Show the window
        ShowWindow(hwnd, SW_SHOWMAXIMIZED);
        UpdateWindow(hwnd);
        
        Ok(Self { hwnd })
    }
    
    fn update(&self) {
        // Increment frame counter
        FRAME_COUNTER.fetch_add(1, Ordering::SeqCst);
        
        unsafe {
            InvalidateRect(self.hwnd, None, false);
            UpdateWindow(self.hwnd);
        }
    }
    
    unsafe extern "system" fn wnd_proc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        match msg {
            WM_PAINT => {
                let mut ps: PAINTSTRUCT = zeroed();
                let hdc = BeginPaint(hwnd, &mut ps);
                
                // Get client area dimensions
                let mut rect = RECT::default();
                GetClientRect(hwnd, &mut rect);
                
                // Create a memory DC and bitmap for double-buffering
                let mem_dc = CreateCompatibleDC(hdc);
                let mem_bmp = CreateCompatibleBitmap(hdc, rect.right, rect.bottom);
                let old_bmp = SelectObject(mem_dc, mem_bmp);
                
                // Fill background with dark color
                let brush = CreateSolidBrush(COLORREF(0x00280A)); // Dark blue
                FillRect(mem_dc, &rect, brush);
                DeleteObject(brush);
                
                // Draw a simple loading animation
                let center_x = rect.right / 2;
                let center_y = rect.bottom / 2;
                
                // Get current frame counter
                let frame = FRAME_COUNTER.load(Ordering::SeqCst);
                
                // Create white brush for dots
                let white_brush = CreateSolidBrush(COLORREF(0xFFFFFF)); // White
                
                // Draw spinning dots
                for i in 0..8 {
                    let angle = ((frame as f64) * 0.05) + (i as f64 * std::f64::consts::PI / 4.0);
                    let distance = 50.0;
                    let x = center_x + (distance * angle.cos()) as i32;
                    let y = center_y + (distance * angle.sin()) as i32;
                    
                    // Determine dot size based on position
                    let dot_size = if i == (frame / 10) as usize % 8 { 8 } else { 4 };
                    
                    // Create a rect for each dot
                    let dot_rect = RECT {
                        left: x - dot_size,
                        top: y - dot_size,
                        right: x + dot_size,
                        bottom: y + dot_size,
                    };
                    
                    // Fill the dot
                    FillRect(mem_dc, &dot_rect, white_brush);
                }
                
                DeleteObject(white_brush);
                
                // Copy from memory DC to window DC
                BitBlt(hdc, 0, 0, rect.right, rect.bottom, mem_dc, 0, 0, SRCCOPY);
                
                // Clean up
                SelectObject(mem_dc, old_bmp);
                DeleteObject(mem_bmp);
                DeleteDC(mem_dc);
                
                EndPaint(hwnd, &ps);
                LRESULT(0)
            }
            WM_DESTROY => {
                PostQuitMessage(0);
                LRESULT(0)
            }
            _ => DefWindowProcW(hwnd, msg, wparam, lparam),
        }
    }
    
    fn process_messages(&self) -> bool {
        unsafe {
            let mut msg: MSG = zeroed();
            while PeekMessageW(&mut msg, None, 0, 0, PM_REMOVE).as_bool() {
                if msg.message == WM_QUIT {
                    return false;
                }
                TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }
        }
        true
    }
    
    fn close(&self) {
        unsafe {
            DestroyWindow(self.hwnd);
        }
    }
}

fn main() -> Result<(), String> {
    // Parse command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err("Usage: splash_launcher.exe path/to/program.exe".to_string());
    }
    
    let exe_path = &args[1];
    println!("Launching: {}", exe_path);
    
    // Create the splash window
    let splash = unsafe { SplashWindow::create()? };
    
    // Launch the target application
    let mut child = Command::new(exe_path)
        .spawn()
        .map_err(|e| format!("Failed to launch program: {}", e))?;
    
    let pid = child.id();
    println!("Started process with PID: {}", pid);
    
    // Record when we started
    let start_time = Instant::now();
    
    // Keep showing the splash screen for at least 2 seconds
    let min_display_time = Duration::from_secs(2);
    
    // Main loop - show the splash window for the minimum time
    let mut running = true;
    
    println!("Showing splash screen for at least 2 seconds...");
    
    while running {
        // Process Windows messages
        running = splash.process_messages();
        if !running {
            break;
        }
        
        // Update animation
        splash.update();
        
        // Check if we've shown the splash for at least the minimum time
        if start_time.elapsed() >= min_display_time {
            // Check if the game process is still running
            match child.try_wait() {
                Ok(Some(status)) => {
                    println!("Process has exited with status: {}", status);
                    // If the process has exited, exit the splash screen too
                    break;
                },
                Ok(None) => {
                    // Process is still running, we can exit the splash screen now
                    println!("Process is running and minimum time elapsed - closing splash screen");
                    break;
                },
                Err(e) => {
                    println!("Error checking process status: {}", e);
                    // If we can't check the process, assume it's running and exit
                    break;
                }
            }
        }
        
        // Sleep a bit to limit CPU usage
        thread::sleep(Duration::from_millis(16)); // ~60 FPS
    }
    
    // Clean up
    splash.close();
    
    Ok(())
}