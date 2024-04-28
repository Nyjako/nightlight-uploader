use std::{env, io::Cursor, thread, time::{Duration, Instant}};
use xcap::{image::{ImageBuffer, Rgba}, image, Window};
use global_hotkey::{hotkey::{Code, HotKey, Modifiers}, GlobalHotKeyEvent, GlobalHotKeyManager, HotKeyState};
use reqwest::{blocking::{multipart::{Form, Part}, Client}, header::HeaderMap};
use chrono::Local;
use serde_json;

const API_KEY: &str = env!("NIGHTLIGHT_API_KEY");
const API_URL: &str = "https://api.nightlight.gg/v1/upload";

fn try_capture_window(name: String) -> Option<ImageBuffer<Rgba<u8>, Vec<u8>>> {
    let windows = Window::all().expect("Failed to get vector of windows.");
    for win in windows {
        if win.is_minimized() { continue; }
        if win.title().to_lowercase().contains(name.to_lowercase().as_str()) {
            return Some(win.capture_image().expect("Failed to capture screenshot of window"));
        }
    }

    None
}

fn upload_screenshot(buffer: &ImageBuffer<Rgba<u8>, Vec<u8>>) -> bool {
    let client = Client::new();
    
    let dynamic_image: image::DynamicImage = image::DynamicImage::ImageRgba8(buffer.to_owned());
    let mut png_data: Vec<u8> = Vec::new();
    let mut cursor = Cursor::new(&mut png_data);
    dynamic_image.write_to(&mut cursor, image::ImageFormat::Png)
        .expect("Failed to convert captured image to png buffer.");

    let form = Form::new().part(
        "file", 
        Part::bytes(png_data).file_name("image.png")
    );

    let mut headers = HeaderMap::new();
    headers.insert(
        "Authorization", 
        format!("Bearer {}", API_KEY).parse().expect("Failed to parse!")
    );

    let request = client.post(API_URL)
        .headers(headers)
        .multipart(form);

    if let Ok(response) = request.send() {
        let status = response.status().as_u16();
        let json: serde_json::Value = response.json().expect("Failed to read response as json!");
        match status {
            200 => {
                if let Some(url) = json["data"]["url"].as_str() {
                    println!("URL: {}", url);
                } else {
                    println!("No URL found in response");
                    return false;
                }
            }
            400 | 403 => {
                if let Some(message) = json["error"]["message"].as_str() {
                    println!("Error Message: {}", message);
                } else {
                    println!("No error message found in response");
                    return false;
                }
                if let Some(status) = json["error"]["status"].as_str() {
                    println!("Error Status: {}", status);
                } else {
                    println!("No error status found in response");
                    return false;
                }
            }
            _ => {
                println!("Unexpected response status code: {}", status);
                return false;
            }
        }
    } else {
        return false;
    }

    true
}

fn capture_and_upload() -> () {
    println!("Trying to capture Dead by daylight window");
    if let Some(buffer) = try_capture_window("DeadByDaylight".to_string()) {
        println!("Trying to upload screenshot");
        if !upload_screenshot(&buffer) {
            eprintln!("Request failed!");
            println!("Trying to save screenshot");
            let output_name = format!("{}.png", Local::now().format("%Y.%m.%d %H-%M-%S"));
            match env::current_exe() {
                Ok(exe_path) => {
                    let path = exe_path.parent().expect("Failed to extract path from filepath!")
                                             .to_str().expect("Failed to convert path to str");
                    let output_path = format!("{}/{}", path, output_name);
                    let _ = buffer.save(output_path).map_err(|err|{
                        eprintln!("Failed to save image:\n{}", err);
                    });
                }
                Err(e) => {
                    println!("Failed to get current exe path: {e}\n");
                    println!("Trying to save screenshot in CWD");
                    let output_path = format!("./{}", output_name);
                    let _ = buffer.save(output_path).map_err(|err|{
                        eprintln!("Failed to save image:\n{}", err);
                    });
                }
            };
        }
    } else {
        eprintln!("Failed to capture Dead by daylight window!");
    }

}

fn main() {
    let start = Instant::now();

    let manager = GlobalHotKeyManager::new().expect("Failed to initialize global hotkey.");
    let capure_hotkey = HotKey::new(Some(Modifiers::CONTROL), Code::Numpad1);
    let exit_hotkey   = HotKey::new(Some(Modifiers::CONTROL), Code::Numpad2);
    manager.register(capure_hotkey).expect("Failed to register capure hotkey!");
    manager.register(exit_hotkey).expect("Failed to register exit hotkey!");

    println!("App has started!");
    let mut app_running = true;
    while app_running {
        if let Ok(event) = GlobalHotKeyEvent::receiver().try_recv() {
            if event.state == HotKeyState::Pressed {
                if event.id == exit_hotkey.id() {
                    app_running = false;
                    continue;
                }
                else if event.id == capure_hotkey.id() {
                    capture_and_upload();
                }
            }
        }

        // We can wait a bit
        thread::sleep( Duration::from_millis(100) );
    }

    println!("Elapsed: {:?}", start.elapsed());
}
