use std::collections::BTreeMap;
use zellij_tile::prelude::*;

#[no_mangle]
pub extern "C" fn _start() {}

#[derive(Default)]
struct VimNav {
    pane_info: Option<PaneInfo>,
}

register_plugin!(VimNav);

impl ZellijPlugin for VimNav {
    fn load(&mut self, _configuration: BTreeMap<String, String>) {
        request_permission(&[
            PermissionType::ReadApplicationState,
            PermissionType::ChangeApplicationState,
        ]);
        subscribe(&[EventType::PaneUpdate]);
    }

    fn update(&mut self, event: Event) -> bool {
        if let Event::PaneUpdate(pane_manifest) = event {
            for (_tab_id, panes) in pane_manifest.panes {
                for pane in panes {
                    if pane.is_focused && !pane.is_plugin {
                        self.pane_info = Some(pane);
                        break;
                    }
                }
            }
        }
        false
    }

    fn pipe(&mut self, pipe_message: PipeMessage) -> bool {
        // If vim, do nothing - the Write escape sequences handle it
        // If not vim, use zellij move-focus
        if !self.is_vim() {
            match pipe_message.name.as_str() {
                "left" => move_focus(Direction::Left),
                "down" => move_focus(Direction::Down),
                "up" => move_focus(Direction::Up),
                "right" => move_focus(Direction::Right),
                _ => {}
            }
        }
        false
    }
}

impl VimNav {
    fn is_vim(&self) -> bool {
        if let Some(ref pane) = self.pane_info {
            // Check terminal command
            if let Some(ref cmd) = pane.terminal_command {
                let cmd_lower = cmd.to_lowercase();
                if cmd_lower.contains("nvim")
                    || cmd_lower.contains("vim")
                    || cmd_lower.ends_with("/vi")
                {
                    return true;
                }
            }
            // Check pane title (nvim usually sets this)
            let title = pane.title.to_lowercase();
            if title.contains("nvim") || title.contains("vim") {
                return true;
            }
        }
        false
    }
}
