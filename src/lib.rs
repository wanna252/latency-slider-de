#![feature(restricted_std)]

use skyline::hooks::InlineCtx;
use skyline::nn::ui2d::{Layout, Pane};
use smash::ui2d::{SmashPane, SmashTextBox};

#[skyline::from_offset(0x37a1ef0)]
unsafe fn set_text_string(pane: u64, string: *const u8);

static mut CURRENT_PANE_HANDLE: usize = 0;
static mut CURRENT_ARENA_ID: String = String::new();
static mut CURRENT_INPUT_BUFFER: isize = 3; // 常に3フレームに固定
static mut MOST_RECENT_AUTO: isize = -1;
static mut STEALTH_MODE: bool = true; // 常にステルスモード
static mut ORIG_VIP_TEXT: String = String::new();
static mut IS_CSS: bool = false;

const MAX_INPUT_BUFFER: isize = 25;
const MIN_INPUT_BUFFER: isize = -1;

struct DpadInputState {
    left_released: bool,
    right_released: bool,
    up_released: bool,
    down_released: bool,
}

static mut DPAD: DpadInputState = DpadInputState {
    left_released: true,
    right_released: true,
    up_released: true,
    down_released: true,
};

unsafe fn handle_user_input(on_css: bool) {
    // D-pad入力によるCURRENT_INPUT_BUFFERの変更は無効化
    STEALTH_MODE = true; // 常にステルスモード
}

#[skyline::hook(offset = 0x18881d0, inline)]
unsafe fn non_hdr_update_room_hook(_: &skyline::hooks::InlineCtx) {
    handle_user_input(false);

    if IS_CSS {
        IS_CSS = false;
    }

    if STEALTH_MODE {
        set_text_string(
            CURRENT_PANE_HANDLE as u64,
            format!("ID: {}\0", CURRENT_ARENA_ID).as_ptr(),
        );
    } else {
        set_text_string(
            CURRENT_PANE_HANDLE as u64,
            format!(
                "ID: {}\nInput Latency: {}\0",
                CURRENT_ARENA_ID, CURRENT_INPUT_BUFFER
            )
            .as_ptr(),
        );
    }
}

#[skyline::hook(offset = 0x004b620)]
unsafe fn handle_draw_hook(layout: *mut Layout, draw_info: u64, cmd_buffer: u64) {
    if IS_CSS {
        let root_pane = &mut *(*layout).root_pane;

        draw_ui(&root_pane);
    }

    call_original!(layout, draw_info, cmd_buffer);
}

