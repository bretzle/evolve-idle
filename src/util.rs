use imgui::sys::*;
use std::ptr;

// Code adapted from https://github.com/ocornut/imgui/issues/3518
struct StatusBar;

impl StatusBar {
    #[must_use]
    unsafe fn begin() -> Option<StatusBarToken> {
        let name = b"##MainStatusBar";

        let g = igGetCurrentContext();
        let viewport = *(*g).Viewports.Data;
        let mut menu_bar_window = igFindWindowByName(name.as_ptr().cast());

        // For the main menu bar, which cannot be moved, we honor g.Style.DisplaySafeAreaPadding to ensure text can be visible on a TV set.
        (*g).NextWindowData.MenuBarOffsetMinVal = ImVec2 {
            x: (*g).Style.DisplaySafeAreaPadding.x,
            y: f32::max((*g).Style.DisplaySafeAreaPadding.y - (*g).Style.FramePadding.y, 0.0),
        };

        fn add(a: ImVec2, b: ImVec2) -> ImVec2 {
            ImVec2 { x: b.x, y: a.y - b.y }
        }

        // Get our rectangle at the bottom of the work area
        //__debugbreak();
        if menu_bar_window.is_null() || (*menu_bar_window).BeginCount == 0 {
            // Set window position
            // We don't attempt to calculate our height ahead, as it depends on the per-viewport font size. However menu-bar will affect the minimum window size so we'll get the right height.
            let menu_bar_pos = add((*viewport)._ImGuiViewport.Size, (*viewport).WorkOffsetMin);
            let menu_bar_size = ImVec2 {
                x: (*viewport)._ImGuiViewport.Size.x - (*viewport).WorkOffsetMin.x + (*viewport).WorkOffsetMax.x,
                y: 1.0,
            };
            igSetNextWindowPos(menu_bar_pos, 0, ImVec2::default()); // verify operator overloading
            igSetNextWindowSize(menu_bar_size, 0);
        }

        // Create window
        igSetNextWindowViewport((*viewport)._ImGuiViewport.ID); // Enforce viewport so we don't create our own viewport when ImGuiConfigFlags_ViewportsNoMerge is set.

        igPushStyleVarFloat(ImGuiStyleVar_WindowRounding as i32, 0.0);
        igPushStyleVarVec2(ImGuiStyleVar_WindowMinSize as i32, ImVec2 { x: 0.0, y: 0.0 }); // Lift normal size constraint, however the presence of a menu-bar will give us the minimum height we want.
        let window_flags = ImGuiWindowFlags_NoDocking
            | ImGuiWindowFlags_NoTitleBar
            | ImGuiWindowFlags_NoResize
            | ImGuiWindowFlags_NoMove
            | ImGuiWindowFlags_NoScrollbar
            | ImGuiWindowFlags_NoSavedSettings
            | ImGuiWindowFlags_MenuBar;

        let is_open = igBegin(name.as_ptr().cast(), ptr::null_mut(), window_flags as i32) && igBeginMenuBar();
        igPopStyleVar(2);

        // Report our size into work area (for next frame) using actual window size
        menu_bar_window = igGetCurrentWindow();
        if (*menu_bar_window).BeginCount == 1 {
            (*viewport).WorkOffsetMin.y += (*menu_bar_window).Size.y;
        }

        (*g).NextWindowData.MenuBarOffsetMinVal = ImVec2 { x: 0.0, y: 0.0 };
        if !is_open {
            igEnd();
            return None;
        }

        return Some(StatusBarToken);
    }
}

pub fn statusbar<R, F: FnOnce() -> R>(f: F) -> Option<R> {
    unsafe { StatusBar::begin().map(|_| f()) }
}

pub struct StatusBarToken;

impl Drop for StatusBarToken {
    fn drop(&mut self) {
        unsafe {
            igEndMenuBar();

            // When the user has left the menu layer (typically: closed menus through activation of an item), we restore focus to the previous window
            // FIXME: With this strategy we won't be able to restore a NULL focus.
            let g = igGetCurrentContext();
            if (*g).CurrentWindow == (*g).NavWindow && (*g).NavLayer == ImGuiNavLayer_Main && !(*g).NavAnyRequest {
                igFocusTopMostWindowUnderOne((*g).NavWindow, ptr::null_mut());
            }

            igEnd();
        }
    }
}
