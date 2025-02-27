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
    
    // Helper function to draw a rocket
    unsafe fn draw_rocket(mem_dc: HDC, center_x: i32, center_y: i32, frame: i32) {
        // Create pens and brushes
        let rocket_pen = CreatePen(PS_SOLID, 2, COLORREF(0xFFFFFF)); // White pen for outlines
        let rocket_brush = CreateSolidBrush(COLORREF(0xCCCCCC)); // Light gray for rocket body
        let flame_brush1 = CreateSolidBrush(COLORREF(0x0066FF)); // Blue flame
        let flame_brush2 = CreateSolidBrush(COLORREF(0xFF6600)); // Orange flame
        
        // Select pen and set background mode to transparent
        let old_pen = SelectObject(mem_dc, rocket_pen);
        SetBkMode(mem_dc, TRANSPARENT);
        
        // Calculate rocket dimensions
        let rocket_width = 30;
        let rocket_height = 80;
        let nose_height = 30;
        let fin_width = 15;
        let fin_height = 20;
        
        // Small vertical offset based on frame to give slight hovering effect
        let offset_y = (frame as f64 * 0.1).sin() as i32 * 2;
        
        // Draw the rocket body (rectangle)
        let body_top = center_y - rocket_height / 2 + nose_height / 2 + offset_y;
        let body_bottom = center_y + rocket_height / 2 + offset_y;
        let body_left = center_x - rocket_width / 2;
        let body_right = center_x + rocket_width / 2;
        
        SelectObject(mem_dc, rocket_brush);
        Rectangle(mem_dc, body_left, body_top, body_right, body_bottom);
        
        // Draw the nose cone (triangle)
        let nose_top = body_top - nose_height;
        let nose_left = body_left;
        let nose_right = body_right;
        
        let nose_points = [
            POINT { x: center_x, y: nose_top },
            POINT { x: nose_right, y: body_top },
            POINT { x: nose_left, y: body_top },
        ];
        
        Polygon(mem_dc, &nose_points);
        
        // Draw the left fin
        let fin_points_left = [
            POINT { x: body_left, y: body_bottom - fin_height },
            POINT { x: body_left - fin_width, y: body_bottom },
            POINT { x: body_left, y: body_bottom },
        ];
        
        Polygon(mem_dc, &fin_points_left);
        
        // Draw the right fin
        let fin_points_right = [
            POINT { x: body_right, y: body_bottom - fin_height },
            POINT { x: body_right + fin_width, y: body_bottom },
            POINT { x: body_right, y: body_bottom },
        ];
        
        Polygon(mem_dc, &fin_points_right);
        
        // Draw window on rocket (circle)
        let window_size = 12;
        let window_y = center_y - 10 + offset_y;
        Ellipse(mem_dc, 
            center_x - window_size / 2, 
            window_y - window_size / 2, 
            center_x + window_size / 2, 
            window_y + window_size / 2
        );
        
        // Draw flame at bottom of rocket (animated)
        let flame_phase = (frame / 5) % 4; // 0, 1, 2, or 3
        
        let flame_width_base = 20;
        let flame_height_base = 25;
        // Adjust flame size based on phase
        let flame_width = flame_width_base + flame_phase * 2;
        let flame_height = flame_height_base + flame_phase * 3;
        
        // Alternate between flame colors
        if flame_phase % 2 == 0 {
            SelectObject(mem_dc, flame_brush1);
        } else {
            SelectObject(mem_dc, flame_brush2);
        }
        
        let flame_points = [
            POINT { x: center_x - flame_width / 2, y: body_bottom },
            POINT { x: center_x, y: body_bottom + flame_height },
            POINT { x: center_x + flame_width / 2, y: body_bottom },
        ];
        
        Polygon(mem_dc, &flame_points);
        
        // Draw motion lines around the rocket
        let line_count = 12;
        let max_line_length = 25;
        
        for i in 0..line_count {
            let angle = (i as f64 * 2.0 * std::f64::consts::PI / line_count as f64) + std::f64::consts::PI/4.0;
            // Only draw lines in the bottom 240 degrees to give upward motion effect
            if angle > std::f64::consts::PI / 6.0 && angle < 11.0 * std::f64::consts::PI / 6.0 {
                // Animate line length based on frame
                let phase = (frame + i * 5) % 15;
                let line_length = if phase < 10 { max_line_length * phase / 10 } else { max_line_length * (15 - phase) / 5 };
                
                // Starting point near the rocket
                let start_dist = rocket_width * 3 / 4 + 10;
                let start_x = center_x + (start_dist as f64 * angle.cos()) as i32;
                let start_y = center_y + (start_dist as f64 * angle.sin()) as i32;
                
                // Ending point
                let end_x = start_x + (line_length as f64 * angle.cos()) as i32;
                let end_y = start_y + (line_length as f64 * angle.sin()) as i32;
                
                MoveToEx(mem_dc, start_x, start_y, None);
                LineTo(mem_dc, end_x, end_y);
            }
        }
        
        // Draw "Launching your game" text
        let text = "Launching your game\0".encode_utf16().collect::<Vec<u16>>();
        SetTextColor(mem_dc, COLORREF(0xFFFFFF)); // White text
        
        // Set text properties - just use larger text
        SetBkMode(mem_dc, TRANSPARENT);
        let text_y = center_y + rocket_height / 2 + 50 + offset_y;
        
        // Draw text without custom font
        TextOutW(mem_dc, center_x - 70, text_y, &text[..text.len()-1]);
        
        // Clean up
        SelectObject(mem_dc, old_pen);
        DeleteObject(rocket_pen);
        DeleteObject(rocket_brush);
        DeleteObject(flame_brush1);
        DeleteObject(flame_brush2);
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
                
                // Fill background with dark color (dark blue with a hint of purple)
                let brush = CreateSolidBrush(COLORREF(0x221133)); // Dark blue/purple
                FillRect(mem_dc, &rect, brush);
                DeleteObject(brush);
                
                // Draw the rocket animation
                let center_x = rect.right / 2;
                let center_y = rect.bottom / 2;
                let frame = FRAME_COUNTER.load(Ordering::SeqCst);
                
                // Convert mem_dc to HDC since our draw_rocket function expects HDC
                Self::draw_rocket(hdc, center_x, center_y, frame);
                
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