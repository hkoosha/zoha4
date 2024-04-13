use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use glib::prelude::Cast;

use gtk4::{Application, CssProvider};
use gtk4::ApplicationWindow;
use gtk4::builders::ApplicationBuilder;
use gtk4::builders::ApplicationWindowBuilder;
use gtk4::Label;
use gtk4::Notebook;
use gtk4::prelude::GtkWindowExt;
use gtk4::prelude::WidgetExt;
use gtk4::Widget;
use log::{debug, info};
use log::trace;

use crate::{CSS_CLASS__NOTEBOOK, CSS_CLASS__TRANSPARENT_BG, toggle};
use crate::config::cfg::LastTabExitBehavior;
use crate::config::cfg::TabMode;
use crate::config::cfg::ZohaCfg;
use crate::ui::terminal::ZohaTerminal;
use crate::ZohaCtx;

pub const APPLICATION_ID: &str = "io.koosha.zoha";

pub fn create_application() -> ApplicationBuilder {
    trace!("create app");
    let application: ApplicationBuilder = Application::builder()
        .application_id(APPLICATION_ID)
        ;

    return application;
}

pub fn create_window(cfg: &ZohaCfg,
                     app: &Application) -> ApplicationWindowBuilder {
    let h: u32 = cfg.display.get_height();
    let w: u32 = cfg.display.get_width();

    info!("create window, h={}, w={}", h, w);

    let window = ApplicationWindow::builder()
        .application(app)
        .title(&cfg.display.title)
        .default_width(w as i32)
        .default_height(h as i32)
        .css_classes([CSS_CLASS__TRANSPARENT_BG])
        ;

    return window;
}

pub fn css() -> CssProvider {
    let provider = CssProvider::new();
    provider.load_from_data(
        &format!("\
        .{} {{ background-color: transparent; }}
        .{} > * {{ background-color: transparent; }}
        .{} header {{ background-color: white; }}
        ",
                 CSS_CLASS__TRANSPARENT_BG,
                 CSS_CLASS__TRANSPARENT_BG,
                 CSS_CLASS__NOTEBOOK,
        )
    );

    return provider;
}

pub fn init_window(ctx: &mut ZohaCtx,
                   window: ApplicationWindow) -> eyre::Result<()> {
    trace!("init window");
    ctx.set_window(window.clone())?;

    if ctx.cfg.display.skip_task_bar {
        debug!("skipping taskbar");
        eprintln!("skip taskbar not implemented yet.");
        // match window.surface().downcast::<X11Surface>() {
        //     Ok(mut surface) => {
        //         // unsafe {
        //         // gdk_x11_surface_set_skip_taskbar_hint(
        //         //     surface.as_ptr(),
        //         //     ctx.cfg.display.skip_task_bar.into_glib(),
        //         // );
        //         // }
        //     }
        //     Err(_) => {
        //         match window.surface().downcast::<WaylandSurface>() {
        //             Ok(_wayland) => {
        //                 eprintln!("skip taskbar not implemented for wayland");
        //             }
        //             Err(_) => {
        //                 eprintln!("could not convert surface to either X11 or Wayland")
        //             }
        //         }
        //     }
        // }
    }

    if ctx.cfg.display.always_on_top {
        debug!("always on top");
        eprintln!("always on top not implemented yet");
        // match window.surface().downcast::<X11Surface>() {
        //     Ok(mut surface) => {
        //         // unsafe {
        //         //     surface.xid();
        //         // }
        //     }
        //     Err(_) => {
        //         match window.surface().downcast::<WaylandSurface>() {
        //             Ok(_wayland) => {
        //                 eprintln!("stay on top not implemented for wayland");
        //             }
        //             Err(_) => {
        //                 eprintln!("could not convert surface to either X11 or Wayland")
        //             }
        //         }
        //     }
        // }
    }

    window.set_decorated(false);
    window.set_size_request(0, 0);

    if ctx.cfg.display.sticky {
        debug!("window sticky");
        eprintln!("sticky not implemented");
    } else {
        debug!("window not sticky");
    }

    if ctx.fullscreen {
        debug!("window fullscreen");
        window.fullscreen();
    } else {
        debug!("window not fullscreen");
        window.unfullscreen();
    }

    // For CSS.
    window.set_widget_name("Main");

    return Ok(());
}

pub fn create_notebook(ctx: &mut ZohaCtx) {
    trace!("create notebook");

    let notebook = Notebook::new();

    ctx.set_notebook(notebook);

    let notebook = ctx.get_notebook().unwrap();

    notebook.set_show_border(false);
    notebook.set_scrollable(true);

    notebook.set_tab_pos(ctx.cfg.display.tab_position.to_gtk());

    match ctx.cfg.display.tab_mode {
        TabMode::Always => notebook.set_show_tabs(true),
        TabMode::Auto => notebook.set_show_tabs(false),
        TabMode::Never => notebook.set_show_tabs(false),
    }
}

pub fn on_page_reorder(ctx: &Rc<RefCell<ZohaCtx>>,
                       child: &Widget,
                       new_position: u32) {
    let child = match child.downcast_ref::<gtk4::Box>() {
        None => {
            eprintln!("could not get child hbox on pages reorder");
            return;
        }
        Some(hbox) => hbox,
    };

    let old_idx = *match ctx
        .borrow()
        .terminals
        .borrow()
        .iter()
        .find(|(_, term)| term.hbox == *child) {
        None => {
            eprintln!("could not find term on pages reorder");
            return;
        }
        Some((old_idx, _)) => old_idx,
    };

    debug!("reorder: {} => {}", old_idx, new_position);

    let mut new_order = HashMap::new();

    let move_fwd = old_idx < new_position;
    let move_bkw = !move_fwd;

    for (idx, term) in ctx.borrow().terminals.borrow_mut().drain() {
        if idx < old_idx && idx < new_position || old_idx < idx && new_position < idx {
            set_label(ctx, &term, idx);
            new_order.insert(idx, term);
        } else if move_fwd && old_idx != idx {
            set_label(ctx, &term, idx - 1);
            new_order.insert(idx - 1, term);
        } else if move_bkw && old_idx != idx {
            set_label(ctx, &term, idx + 1);
            new_order.insert(idx + 1, term);
        } else {
            set_label(ctx, &term, new_position);
            new_order.insert(new_position, term);
        }
    }

    ctx.borrow().terminals.borrow_mut().extend(new_order);
}

pub fn on_page_removed(ctx: &Rc<RefCell<ZohaCtx>>,
                       page: u32) {
    debug!("page removed: {}", page);

    let cxb = ctx.borrow();
    let mut terminals = cxb.terminals.borrow_mut();

    let adjusted: HashMap<_, _> = terminals
        .drain()
        .map(|(id, term)| {
            let new_idx: u32 = if id < page {
                id
            } else {
                id - 1
            };
            (new_idx, term)
        })
        .collect();

    for (idx, term) in &adjusted {
        set_label(ctx, term, idx + 1);
    }

    terminals.extend(adjusted);
}

pub fn remove_page_by_hbox(ctx: &Rc<RefCell<ZohaCtx>>,
                           hbox: &gtk4::Box) {
    let page: Option<u32> = match ctx.borrow().get_notebook() {
        None => {
            eprintln!("missing notebook on term exit");
            return;
        }
        Some(notebook) => {
            let page: Option<u32> = notebook.page_num(hbox);

            trace!(
                "remove_page_by_hbox, page={}",
                notebook
                .page_num(hbox)
                .map(|it| it.to_string())
                .unwrap_or_else(|| "?".to_string()),
            );

            notebook.remove_page(page);

            page
        }
    };

    if let Some(page) = page {
        ctx
            .borrow()
            .terminals
            .borrow_mut()
            .remove(&page);

        on_page_removed(ctx, page);
    }

    adjust_tab_bar(ctx);
    set_focus(ctx);
}

pub fn add_tab(ctx: &Rc<RefCell<ZohaCtx>>,
               grab_focus: bool) {
    trace!("tab_add, focus: {}", grab_focus);

    let term = ZohaTerminal::new(Rc::clone(ctx));

    let (idx, dir) = match ctx.borrow().get_notebook() {
        None => {
            eprintln!("missing notebook on add tab");
            (None, None)
        }
        Some(notebook) => {
            if notebook.n_pages() > 0 {
                match notebook.current_page() {
                    None => {
                        eprintln!("no active page on notebook on add tab");
                        (None, None)
                    }
                    Some(page) => {
                        match ctx.borrow().terminals.borrow().get(&page) {
                            None => {
                                eprintln!("missing term on add tab: {}", page);
                                (None, None)
                            }
                            Some(term) => {
                                (Some(page + 1), term.get_cwd())
                            }
                        }
                    }
                }
            } else {
                (None, None)
            }
        }
    };

    term.spawn(dir);

    let page: u32 = match ctx.borrow().get_notebook() {
        None => {
            eprintln!("notebook missing on add tab");
            return;
        }
        Some(notebook) => {
            term.connect_signals();

            let new_page_index: u32 = notebook.append_page(
                &term.hbox,
                Some(&Label::new(Some("Zoha"))),
            );
            notebook.set_current_page(Some(new_page_index));
            notebook.set_tab_reorderable(&term.hbox, true);

            if ctx.borrow().cfg.display.tab_expand {
                // notebook.child_set_property(&term.hbox, "tab-expand", &true);
                // notebook.child_set_property(&term.hbox, "tab-fill", &true);
                eprintln!("tab expand not implemented");
            }

            if grab_focus {
                term.vte.grab_focus();
            }

            new_page_index
        }
    };

    adjust_tab_bar(ctx);

    ctx
        .borrow()
        .terminals
        .borrow_mut()
        .insert(page, term.clone());

    if let Some(idx) = idx {
        for _ in idx..page {
            move_tab(ctx, false);
        }
    }

    set_label(ctx, &term, page + 1);
    for (i, t) in ctx.borrow().terminals.borrow().iter() {
        set_label(ctx, t, *i + 1);
    }

    set_focus(ctx);
}

pub fn close_tab(ctx: &Rc<RefCell<ZohaCtx>>) {
    trace!("clos_tab");

    if let Some(mut term) = get_term(ctx, "close_tab") {
        term.kill();
    }

    set_focus(ctx);
}

pub fn move_backward(ctx: &Rc<RefCell<ZohaCtx>>) {
    move_tab(ctx, false);
}

pub fn move_forward(ctx: &Rc<RefCell<ZohaCtx>>) {
    move_tab(ctx, true);
}

pub fn move_tab(ctx: &Rc<RefCell<ZohaCtx>>,
                fwd: bool) {
    trace!("move tab, fwd: {}", fwd);

    if ctx.borrow().get_notebook().is_none() {
        eprintln!("missing notebook on move_tab");
        return;
    }
    if ctx.borrow().get_notebook().unwrap().n_pages() < 1 {
        debug!("no page to reorder on move_tab, skipping");
        return;
    }

    let pages: u32 = ctx.borrow().get_notebook().unwrap().n_pages();

    let idx: u32 = match ctx.borrow().get_notebook().unwrap().current_page() {
        None => {
            debug!("no active page on notebook on move_tab, skipping");
            return;
        }
        Some(page) => page,
    };

    let page: Widget = match ctx.borrow().get_notebook().unwrap().nth_page(Some(idx)) {
        None => {
            eprintln!("could not get notebook page on move_tab at index: {}", idx);
            return;
        }
        Some(page) => page,
    };

    let new_index = match fwd {
        true => (idx + 1) % pages,
        false => if idx == 0 {
            pages
        } else {
            idx - 1
        },
    };

    ctx.borrow().get_notebook().unwrap().reorder_child(&page, Some(new_index));

    set_focus(ctx);
}

pub fn goto_next(ctx: &Rc<RefCell<ZohaCtx>>) {
    trace!("goto_next");

    match ctx.borrow().get_notebook() {
        None => eprintln!("missing notebook on goto next tab"),
        Some(notebook) => {
            if notebook.current_page() == Some(notebook.n_pages() - 1) &&
                ctx.borrow().cfg.display.tab_scroll_wrap {
                notebook.set_current_page(Some(0));
            } else {
                notebook.next_page();
            }
        }
    }

    set_focus(ctx);
}

pub fn goto_previous(ctx: &Rc<RefCell<ZohaCtx>>) {
    trace!("goto_previous");

    match ctx.borrow().get_notebook() {
        None => eprintln!("missing notebook on goto next tab"),
        Some(notebook) => {
            if notebook.current_page() == Some(0) && ctx.borrow().cfg.display.tab_scroll_wrap {
                notebook.set_current_page(Some(notebook.n_pages() - 1));
            } else {
                notebook.prev_page();
            }
        }
    }

    set_focus(ctx);
}

pub fn goto_last(ctx: &Rc<RefCell<ZohaCtx>>) {
    trace!("goto_last");

    match ctx.borrow().get_notebook() {
        None => eprintln!("missing notebook on goto last tab"),
        Some(notebook) => {
            let pages: u32 = notebook.n_pages();
            notebook.set_current_page(Some(pages - 1));
        }
    }

    set_focus(ctx);
}

pub fn goto_n(ctx: &Rc<RefCell<ZohaCtx>>,
              n: usize) {
    trace!("goto_n: {}", n);

    match ctx.borrow().get_notebook() {
        None => eprintln!("missing notebook on goto last tab"),
        Some(notebook) => {
            if n < (notebook.n_pages() as usize) {
                notebook.set_current_page(Some((n - 1) as u32));
            }
        }
    }

    set_focus(ctx);
}

pub fn adjust_tab_bar(ctx: &Rc<RefCell<ZohaCtx>>) {
    match ctx.borrow().get_notebook() {
        None => eprintln!("missing notebook on adjust tabs"),
        Some(notebook) => {
            let num_pages: u32 = notebook.n_pages();

            if ctx.borrow().cfg.display.tab_mode == TabMode::Auto {
                if num_pages < 2 {
                    trace!("hiding tabs");
                    notebook.set_show_tabs(false);
                } else {
                    trace!("showing tabs");
                    notebook.set_show_tabs(true);
                }
            }

            if num_pages == 0 {
                match ctx.borrow().cfg.behavior.last_tab_exit_behavior {
                    LastTabExitBehavior::RestartTerminal => {
                        debug!("adding a tab after last tab close");
                        add_tab(ctx, true);
                    }
                    LastTabExitBehavior::RestartTerminalAndHide => {
                        debug!("adding a tab after last tab close and hiding");
                        add_tab(ctx, false);
                        toggle(ctx);
                    }
                    LastTabExitBehavior::Exit => {
                        if let Some(window) = &ctx.borrow().window {
                            window.close();
                        } else {
                            eprintln!("window missing on exit request on last tab closed")
                        }
                    }
                }
            }
        }
    };
}

pub fn copy(ctx: &Rc<RefCell<ZohaCtx>>) {
    trace!("copy");

    if let Some(zt) = get_term(ctx, "copy") {
        zt.copy();
    }
}

pub fn paste(ctx: &Rc<RefCell<ZohaCtx>>) {
    trace!("paste");

    if let Some(zt) = get_term(ctx, "paste") {
        zt.paste();
    };
}

pub fn toggle_transparency(ctx: &Rc<RefCell<ZohaCtx>>) {
    let en = ctx.borrow().transparency_enabled;
    ctx.borrow_mut().transparency_enabled = !en;

    debug!("toggling transparency, enabled={}", !en);

    ctx
        .borrow()
        .terminals
        .borrow()
        .iter()
        .for_each(|(_, zt)| {
            zt.enforce_transparency();
        });
}

pub fn font_inc(ctx: &Rc<RefCell<ZohaCtx>>) {
    trace!("font_inc");

    ctx
        .borrow_mut()
        .font_inc();

    ctx
        .borrow()
        .terminals
        .borrow()
        .iter()
        .for_each(|(_, zt)| {
            zt.enforce_font_size();
        });
}

pub fn font_dec(ctx: &Rc<RefCell<ZohaCtx>>) {
    trace!("font_dec");

    ctx
        .borrow_mut()
        .font_dec();

    ctx
        .borrow()
        .terminals
        .borrow()
        .iter()
        .for_each(|(_, zt)| {
            zt.enforce_font_size();
        });
}

pub fn font_reset(ctx: &Rc<RefCell<ZohaCtx>>) {
    trace!("font_reset");

    ctx
        .borrow_mut()
        .font_reset();

    ctx
        .borrow()
        .terminals
        .borrow()
        .iter()
        .for_each(|(_, zt)| {
            zt.enforce_font_size();
        });
}

fn get_term(ctx: &Rc<RefCell<ZohaCtx>>,
            action: &'_ str) -> Option<ZohaTerminal> {
    let active_page: u32 = match ctx
        .borrow()
        .get_notebook() {
        None => {
            eprintln!("missing notebook on action callback for: {}", action);
            return None;
        }
        Some(notebook) => match notebook.current_page() {
            None => {
                eprintln!("no active page on notebook on {}", action);
                return None;
            }
            Some(page) => page,
        },
    };

    let term = ctx
        .borrow()
        .terminals
        .borrow()
        .get(&active_page)
        .cloned();

    if term.is_none() {
        eprintln!("missing terminal on action callback for: {}", action);
    }

    return term;
}

fn set_label(ctx: &Rc<RefCell<ZohaCtx>>,
             term: &ZohaTerminal,
             idx: u32) {
    trace!("set_label, idx={}", idx);

    match ctx.borrow().get_notebook() {
        None => {
            eprintln!("missing notebook on page re-order");
        }
        Some(notebook) => {
            match term.get_cwd().map(|it| it.to_string_lossy().to_string()) {
                None => notebook.set_tab_label_text(
                    &term.hbox,
                    &format!("[{}] - {}@?", idx, term.tab_counter),
                ),
                Some(cwd) => {
                    match ctx.borrow().cfg.display.tab_title_num_characters {
                        None => notebook.set_tab_label_text(
                            &term.hbox,
                            &format!("[{}] - {}@{}", idx, term.tab_counter, cwd.as_str()),
                        ),
                        Some(chars) => {
                            if cwd.len() <= chars.unsigned_abs() as usize {
                                notebook.set_tab_label_text(
                                    &term.hbox,
                                    &format!(
                                        "[{}] - {}@{}",
                                        idx, term.tab_counter, cwd.as_str(),
                                    ),
                                )
                            } else if chars > 0 {
                                notebook.set_tab_label_text(
                                    &term.hbox,
                                    &format!(
                                        "[{}] - {}@{}",
                                        idx,
                                        term.tab_counter,
                                        &cwd
                                            .as_str()
                                            [(cwd.len() - (chars.unsigned_abs() as usize))..cwd.len()],
                                    ),
                                );
                            } else {
                                notebook.set_tab_label_text(
                                    &term.hbox,
                                    &format!(
                                        "[{}] - {}@{}",
                                        idx,
                                        term.tab_counter,
                                        &cwd.as_str()[0..(chars.unsigned_abs() as usize)],
                                    ),
                                );
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn set_focus(ctx: &Rc<RefCell<ZohaCtx>>) {
    match ctx.borrow().get_notebook() {
        None => eprintln!("missing notebook on window activate"),
        Some(notebook) => {
            match notebook.current_page() {
                None => {
                    eprintln!("no active page on notebook on window focus");
                }
                Some(page) => {
                    if let Ok(ctx) = ctx.try_borrow() {
                        if let Ok(ctx) = ctx.terminals.try_borrow() {
                            if let Some(term) = ctx.get(&page) {
                                term.vte.grab_focus();
                            }
                        }
                    }
                }
            }
        }
    };
}
