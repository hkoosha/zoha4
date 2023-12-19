use std::cell::RefCell;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::rc::Rc;
use std::string::ToString;

use gdk4::gio;
use gdk4::RGBA;
use glib::ObjectExt;
use glib::Pid;
use glib::SignalHandlerId;
use glib::SpawnFlags;
use gtk4::Orientation;
use gtk4::prelude::ScrollableExt;
use gtk4::prelude::WidgetExt;
use gtk4::Scrollbar;
use gtk4::traits::BoxExt;
use log::debug;
use vte4::{Format, TerminalExt, TerminalExtManual};
use vte4::PtyFlags;
use vte4::Terminal;

use crate::config::cfg::ScrollbarPosition;
use crate::config::cfg::TerminalExitBehavior;
use crate::ui::window::remove_page_by_hbox;
use crate::ZohaCtx;

struct ZohaTerminalCtx {
    ctx: Rc<RefCell<ZohaCtx>>,
    pid: Option<Pid>,
    // dropped_to_default_shell: bool,
    hbox: gtk4::Box,
    exit_handler: Option<SignalHandlerId>,
}


#[derive(Clone)]
pub struct ZohaTerminal {
    pub hbox: gtk4::Box,
    pub vte: Terminal,
    pub scrollbar: Scrollbar,
    pub tab_counter: usize,
    ctx: Rc<RefCell<ZohaTerminalCtx>>,
}

impl Debug for ZohaTerminal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ZohaTerminal[pid={:?}]",
               self.ctx
                   .try_borrow()
                   .map(|it|
                       it.pid.map(|pid| format!("{:?}", pid)).unwrap_or_else(|| "?".to_string())
                   )
                   .unwrap_or_else(|_| "?".to_string())
        )
    }
}

impl ZohaTerminal {
    pub fn new(ctx: Rc<RefCell<ZohaCtx>>) -> Self {
        let vte: Terminal = {
            let cfg = &ctx.borrow().cfg;

            let vte: Terminal = Terminal::new();

            vte.set_cursor_blink_mode(cfg.terminal.cursor_blink_to_vte());
            vte.set_cursor_shape(cfg.terminal.cursor_shape.to_vte());

            vte.set_scroll_on_keystroke(cfg.terminal.scroll_on_keystroke);
            vte.set_scroll_on_output(cfg.terminal.scroll_on_output);
            vte.set_scrollback_lines(cfg.terminal.scrollback_lines);

            vte.set_backspace_binding(cfg.terminal.backspace_binding.to_vte());
            vte.set_delete_binding(cfg.terminal.delete_binding.to_vte());

            vte.set_mouse_autohide(cfg.terminal.mouse_auto_hide);

            vte.set_allow_hyperlink(cfg.terminal.allow_hyper_link);
            vte.set_word_char_exceptions(&cfg.terminal.word_char_exceptions);
            vte.set_audible_bell(cfg.terminal.audible_bell);

            vte.set_font(Some(&cfg.font.font));

            vte.set_color_cursor(Some(&cfg.color.cursor));
            vte.set_color_cursor_foreground(Some(&cfg.color.bg));

            let owned: Vec<RGBA> = cfg.color.user_pallet().into_iter().collect();
            let collected: Vec<&RGBA> = owned.iter().collect();
            vte.set_colors(
                Some(&cfg.color.fg),
                Some(&cfg.color.bg),
                &collected,
            );

            vte.add_css_class("transparent-bg");

            vte
        };

        let scrollbar = Scrollbar::new(
            Orientation::Vertical,
            vte.vadjustment().as_ref(),
        );

        let hbox = gtk4::Box::new(Orientation::Horizontal, 0);
        hbox.add_css_class("transparent-bg");
        hbox.set_hexpand(true);
        hbox.set_vexpand(true);
        vte.set_hexpand(true);
        vte.set_vexpand(true);

        hbox.append(&scrollbar);
        hbox.append(&vte);
        hbox.set_homogeneous(false);

        match ctx.borrow().cfg.display.scrollbar_position {
            ScrollbarPosition::Left => {
                hbox.reorder_child_after(&vte, Some(&scrollbar));
                scrollbar.show();
            }
            ScrollbarPosition::Right => {
                hbox.reorder_child_after(&scrollbar, Some(&vte));
                scrollbar.show();
            }
            ScrollbarPosition::Hidden => scrollbar.hide(),
        }

        let tab_counter: usize = ctx.borrow().issue_tab_number();

        let term_ctx = Rc::new(RefCell::new(ZohaTerminalCtx {
            ctx,
            pid: None,
            // dropped_to_default_shell: false,
            hbox: hbox.clone(),
            exit_handler: None,
        }));

        return Self {
            hbox,
            vte,
            scrollbar,
            ctx: term_ctx,
            tab_counter,
        };
    }

    pub fn connect_signals(&self) {
        let ctx = Rc::clone(&self.ctx);
        let ctx0 = Rc::clone(&ctx);

        let handler: SignalHandlerId = self.vte.connect_child_exited(move |vte, _| {
            let mut cxb = ctx.borrow_mut();
            let behavior: TerminalExitBehavior =
                cxb.ctx.borrow().cfg.behavior.terminal_exit_behavior;
            match behavior {
                // TerminalExitBehavior::DropToDefaultShell => {
                //     todo!("DropToDefaultShell");
                // }
                // TerminalExitBehavior::RestartCommand => {
                //     todo!("RestartCommand");
                // }
                TerminalExitBehavior::ExitTerminal => {
                    let handler = cxb.exit_handler.take();
                    match handler {
                        None => eprintln!("missing exit signal handler"),
                        Some(handler) => vte.disconnect(handler),
                    };
                    remove_page_by_hbox(&cxb.ctx, &cxb.hbox);
                }
            };
        });

        ctx0.borrow_mut().exit_handler = Some(handler);
    }

    pub fn kill(&mut self) {
        debug!("killing terminal: {}",
            self
            .ctx
            .try_borrow()
            .map(|it| it.pid.map(|it| it.0.to_string()).unwrap_or_else(|| "?".to_string()))
            .unwrap_or_else(|_| "?".to_string())
        );

        // Can not happen in close_terminal as ctx is already borrowed there from some other
        // call paths.
        match self.ctx.borrow_mut().exit_handler.take() {
            None => eprintln!("missing exit signal handler"),
            Some(handler) => self.vte.disconnect(handler),
        };

        remove_page_by_hbox(&self.ctx.borrow().ctx, &self.ctx.borrow().hbox);
    }

    pub fn spawn(&self,
                 working_dir: Option<PathBuf>) {
        let dir: Option<String> = working_dir.or_else(||
            self.ctx
                .borrow()
                .ctx
                .borrow()
                .cfg
                .process
                .working_dir
                .as_ref()
                .map(PathBuf::from)
        ).map(|it| it.into_os_string().to_string_lossy().into_owned());

        let callback_ctx = Rc::clone(&self.ctx);

        self.vte.spawn_async(
            PtyFlags::DEFAULT,
            dir.as_deref(),
            &[&self.ctx.borrow().ctx.borrow().cfg.process.command],
            &[],
            SpawnFlags::DEFAULT,
            || {},
            10,
            None::<&gio::Cancellable>,
            move |result| {
                match result {
                    Ok(pid) => { callback_ctx.borrow_mut().pid = Some(pid) }
                    Err(err) => {
                        eprintln!("could not spawn terminal: {}", err);
                    }
                }
            },
        );

        self.enforce_font_size();
    }

    pub fn get_cwd(&self) -> Option<PathBuf> {
        self.ctx.borrow().pid?;

        let pid: i32 = self.ctx.borrow().pid.as_ref().unwrap().0;
        let cwd_path: String = format!("/proc/{}/cwd", pid);
        let path = match fs::read_link(Path::new(&cwd_path)) {
            Ok(path) => path,
            Err(err) => {
                eprintln!("could not get working directory: {}, pid={}", err, pid);
                return None;
            }
        };

        return Some(path);
    }

    pub fn copy(&self) {
        // TODO move format to cfg.
        self.vte.copy_clipboard_format(Format::Text);
    }

    pub fn paste(&self) {
        self.vte.paste_clipboard();
    }

    pub fn enforce_transparency(&self) {
        let enabled: bool = self.ctx.borrow().ctx.borrow().transparency_enabled;

        let mut bg: RGBA = self.ctx.borrow().ctx.borrow().cfg.color.bg;
        if !enabled {
            bg.set_alpha(1.0);
        }

        let owned: Vec<RGBA> =
            self.ctx.borrow().ctx.borrow().cfg.color.user_pallet().into_iter().collect();
        let collected: Vec<&RGBA> = owned.iter().collect();
        self.vte.set_colors(
            Some(&self.ctx.borrow().ctx.borrow().cfg.color.fg),
            Some(&bg),
            &collected,
        );
    }

    pub fn enforce_font_size(&self) {
        let scale: f64 = self.ctx.borrow().ctx.borrow().font_scale;
        self.vte.set_font_scale(scale);
    }
}
