use glfw;
use log;

use crate::cores::common;

// todo
// https://github.com/raysan5/raylib/blob/803b1a910e0b23986688e9d4e53b77d18ba41767/src/platforms/rcore_desktop.c#L52-L95

pub struct PlatformData {
    pub handle: *mut glfw::ffi::GLFWwindow,
}

// todo
// extern Coredata CORE;

// todo
// const platform = PlatformData {};

/// Check if application should close
pub fn WindowShouldClose(core: &common::CoreData) -> bool {
    // NOTE: By default, if KEY_ESCAPE pressed or window close icon clicked
    match core.window.ready {
        true => return core.window.shouldClose,
        false => true,
    }
}

pub fn GetCurrentMonitor(core: &common::CoreData, platform: &PlatformData) -> i32 {
    let mut index = 0;
    let mut monitor_count = 0;
    let monitors: *mut *mut glfw::ffi::GLFWmonitor;
    let mut monitor: *mut glfw::ffi::GLFWmonitor;

    unsafe {
        monitors = glfw::ffi::glfwGetMonitors(&mut monitor_count);
    }

    if monitor_count >= 1 {
        if common::IsWindowFullscreen(core) {
            unsafe { monitor = glfw::ffi::glfwGetWindowMonitor(platform.handle) }
            for i in 0..monitor_count {
                unsafe {
                    // good lord holy shit
                    // todo, test
                    if *monitors.offset(i as isize) == monitor {
                        index = i;
                        break;
                    }
                }
            }
        } else {
            let mut x = 0;
            let mut y = 0;

            unsafe {
                glfw::ffi::glfwGetWindowPos(platform.handle, &mut x, &mut y);
            }

            for i in 0..monitor_count {
                let mut mx = 0;
                let mut my = 0;
                let mode: *const glfw::ffi::GLFWvidmode;

                unsafe {
                    monitor = *monitors.offset(i as isize);
                    glfw::ffi::glfwGetMonitorPos(monitor, &mut mx, &mut my);
                    mode = glfw::ffi::glfwGetVideoMode(monitor);
                }

                if mode == std::ptr::null() {
                    let safemode = unsafe { &*mode };
                    let width = safemode.width;
                    let height = safemode.height;

                    if (x >= mx) && (x < (mx + width)) && (y >= my) && (y < (my + height)) {
                        index = i;
                        break;
                    }
                } else {
                    log::trace!("GLFW: Failed to find video mode for selected monitor");
                }
            }
        }
    }

    index
}

/// Toggle fullscreen mode
pub fn ToggleFullscreen(
    core: &mut common::CoreData,
    window: &glfw::Window,
    platform: &PlatformData,
) {
    if !core.window.fullscreen {
        // Store previous window position (in case we exit fullscreen)
        let (x, y) = window.get_pos();
        core.window.position.x = x;
        core.window.position.y = y;

        let mut monitor_count = 0;
        let monitor_index = GetCurrentMonitor(core, platform);

        let monitors: *mut *mut glfw::ffi::GLFWmonitor;
        let monitor: *mut glfw::ffi::GLFWmonitor;

        unsafe {
            monitors = glfw::ffi::glfwGetMonitors(&mut monitor_count);

            monitor = match monitor_index < monitor_count {
                true => *monitors.offset(monitor_index as isize),
                false => std::ptr::null_mut(),
            };
        }

        if monitor == std::ptr::null_mut() {
            log::warn!("GLFW: Failed to get monitor");
            core.window.fullscreen = false;
            // todo
            // core.window.flags &= ~FLAG_FULLSCREEN_MODE;
            unsafe {
                glfw::ffi::glfwSetWindowMonitor(
                    platform.handle,
                    monitor,
                    0,
                    0,
                    core.window.screen.width as std::os::raw::c_int,
                    core.window.screen.height as std::os::raw::c_int,
                    glfw::ffi::DONT_CARE,
                );
            }
        } else {
            core.window.fullscreen = true;
            // todo
            // core.window.flags |= FLAG_FULLSCREEN_MODE;
            unsafe {
                glfw::ffi::glfwSetWindowMonitor(
                    platform.handle,
                    monitor,
                    0,
                    0,
                    core.window.screen.width as std::os::raw::c_int,
                    core.window.screen.height as std::os::raw::c_int,
                    glfw::ffi::DONT_CARE,
                );
            }
        }
    } else {
        core.window.fullscreen = false;
        // todo
        // core.window.flags &= ~FLAG_FULLSCREEN_MODE;
        unsafe {
            glfw::ffi::glfwSetWindowMonitor(
                platform.handle,
                std::ptr::null_mut(),
                core.window.position.x,
                core.window.position.y,
                core.window.screen.width as std::os::raw::c_int,
                core.window.screen.height as std::os::raw::c_int,
                glfw::ffi::DONT_CARE,
            );
        }
    }

    // todo
    // Try to enable GPU V-Sync, so frames are limited to screen refresh rate (60Hz -> 60 FPS)
    // NOTE: V-Sync can be enabled by graphic driver configuration
    // if core.window.flags & FLAG_VSYNC_HINT {
    //     unsafe {
    //         glfw::ffi::glfwSwapInterval(1);
    //     }
    // }
}
