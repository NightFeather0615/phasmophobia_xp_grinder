use std::{thread, time::Duration, mem::size_of};

use anyhow::Result;
use windows::Win32::UI::{
  WindowsAndMessaging::SetCursorPos,
  Input::KeyboardAndMouse::{
    SendInput,
    mouse_event,
    INPUT,
    INPUT_0,
    KEYBDINPUT,
    INPUT_TYPE,
    VIRTUAL_KEY,
    KEYEVENTF_KEYUP,
    MOUSEEVENTF_MOVE,
    MOUSEEVENTF_LEFTUP,
    KEYEVENTF_SCANCODE,
    MOUSEEVENTF_LEFTDOWN
  }
};


const ACTION_DELAY: Duration = Duration::from_millis(150);
const GAME_LOADING_DELAY: Duration = Duration::from_secs(20);


unsafe fn run_grind() -> Result<()> {
  // End and Restart

  thread::sleep(ACTION_DELAY);
  SetCursorPos(350, 750)?; // Skip
  thread::sleep(ACTION_DELAY);
  mouse_event(MOUSEEVENTF_LEFTDOWN, 350, 750, 0, 0);
  thread::sleep(ACTION_DELAY);
  mouse_event(MOUSEEVENTF_LEFTUP, 350, 750, 0, 0);

  thread::sleep(ACTION_DELAY);
  SetCursorPos(760, 600)?; // Confirm Unlocked Item
  thread::sleep(ACTION_DELAY);
  mouse_event(MOUSEEVENTF_LEFTDOWN, 760, 600, 0, 0);
  thread::sleep(ACTION_DELAY);
  mouse_event(MOUSEEVENTF_LEFTUP, 760, 600, 0, 0);

  thread::sleep(ACTION_DELAY);
  SetCursorPos(1175, 750)?; // Next
  thread::sleep(ACTION_DELAY);
  mouse_event(MOUSEEVENTF_LEFTDOWN, 1175, 750, 0, 0);
  thread::sleep(ACTION_DELAY);
  mouse_event(MOUSEEVENTF_LEFTUP, 1175, 750, 0, 0);


  // Choose Map
  
  thread::sleep(ACTION_DELAY);
  SetCursorPos(80, 440)?; // Left Panel
  thread::sleep(ACTION_DELAY);
  mouse_event(MOUSEEVENTF_LEFTDOWN, 80, 440, 0, 0);
  thread::sleep(ACTION_DELAY);
  mouse_event(MOUSEEVENTF_LEFTUP, 80, 440, 0, 0);

  thread::sleep(ACTION_DELAY);
  SetCursorPos(1200, 650)?; // Campfire Map
  thread::sleep(ACTION_DELAY);
  mouse_event(MOUSEEVENTF_LEFTDOWN, 1200, 650, 0, 0);
  thread::sleep(ACTION_DELAY);
  mouse_event(MOUSEEVENTF_LEFTUP, 1200, 650, 0, 0);

  thread::sleep(ACTION_DELAY);
  SetCursorPos(1500, 440)?; // Right Panel
  thread::sleep(ACTION_DELAY);
  mouse_event(MOUSEEVENTF_LEFTDOWN, 1500, 440, 0, 0);
  thread::sleep(ACTION_DELAY);
  mouse_event(MOUSEEVENTF_LEFTUP, 1500, 440, 0, 0);


  // Prepare and Start
  
  thread::sleep(ACTION_DELAY);
  SetCursorPos(750, 750)?; // Prepare
  thread::sleep(ACTION_DELAY);
  mouse_event(MOUSEEVENTF_LEFTDOWN, 750, 750, 0, 0);
  thread::sleep(ACTION_DELAY);
  mouse_event(MOUSEEVENTF_LEFTUP, 750, 750, 0, 0);

  thread::sleep(ACTION_DELAY);
  SetCursorPos(1175, 750)?; // Start
  thread::sleep(ACTION_DELAY);
  mouse_event(MOUSEEVENTF_LEFTDOWN, 1175, 750, 0, 0);
  thread::sleep(ACTION_DELAY);
  mouse_event(MOUSEEVENTF_LEFTUP, 1175, 750, 0, 0);


  // Wait for Map Loading

  thread::sleep(GAME_LOADING_DELAY); 


  // Move to Truck Door

  thread::sleep(ACTION_DELAY);
  mouse_event(MOUSEEVENTF_MOVE, 1800, 0, 0, 0);
  thread::sleep(ACTION_DELAY);
  SendInput(
    &[
      INPUT {
        r#type: INPUT_TYPE(1),
        Anonymous: INPUT_0 {
          ki: KEYBDINPUT {
            wVk: VIRTUAL_KEY(0),
            wScan: 0x11,
            dwFlags: KEYEVENTF_SCANCODE,
            time: 0,
            dwExtraInfo: 0
          }
        }
      }
    ],
    size_of::<INPUT>() as i32
  );
  thread::sleep(Duration::from_secs(6));
  SendInput(
    &[
      INPUT {
        r#type: INPUT_TYPE(1),
        Anonymous: INPUT_0 {
          ki: KEYBDINPUT {
            wVk: VIRTUAL_KEY(0),
            wScan: 0x11,
            dwFlags: KEYEVENTF_SCANCODE | KEYEVENTF_KEYUP,
            time: 0,
            dwExtraInfo: 0
          }
        }
      }
    ],
    size_of::<INPUT>() as i32
  );


  // Point to Door Controller

  thread::sleep(ACTION_DELAY);
  mouse_event(MOUSEEVENTF_MOVE, -900, 0, 0, 0);
  thread::sleep(ACTION_DELAY);
  mouse_event(MOUSEEVENTF_MOVE, 0, 130, 0, 0);


  // Open and Close Door
  
  thread::sleep(ACTION_DELAY);
  mouse_event(MOUSEEVENTF_LEFTDOWN, 0, 0, 0, 0);
  thread::sleep(ACTION_DELAY);
  mouse_event(MOUSEEVENTF_LEFTUP, 0, 0, 0, 0);
  thread::sleep(Duration::from_secs(8));
  mouse_event(MOUSEEVENTF_LEFTDOWN, 0, 0, 0, 0);
  thread::sleep(ACTION_DELAY);
  mouse_event(MOUSEEVENTF_LEFTUP, 0, 0, 0, 0);


  // Wait for Game Init

  thread::sleep(GAME_LOADING_DELAY); 


  Ok(())
}

fn main() {
  vec![3, 2, 1].iter().for_each(
    |i| {
      println!("Script start in {}...", i);
      thread::sleep(Duration::from_secs(1));
    }
  );

  let mut loop_count: i128 = 0;
  loop {
    unsafe { run_grind().unwrap() }
    loop_count += 1;
    println!("Workflow was run {} time(s)", loop_count);
  }
}
