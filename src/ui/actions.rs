use std::cell::RefCell;
use std::rc::Rc;

use gdk4::gio::Action;
use glib::clone;
use gtk4::Application;
use gtk4::ApplicationWindow;
use gtk4::gio::SimpleAction;
use gtk4::prelude::ActionMapExt;
use gtk4::prelude::ApplicationWindowExt;
use gtk4::prelude::GtkApplicationExt;
use gtk4::prelude::GtkWindowExt;
use log::debug;

use crate::ui::window::add_tab;
use crate::ui::window::close_tab;
use crate::ui::window::copy;
use crate::ui::window::font_dec;
use crate::ui::window::font_inc;
use crate::ui::window::font_reset;
use crate::ui::window::goto_last;
use crate::ui::window::goto_n;
use crate::ui::window::goto_next;
use crate::ui::window::goto_previous;
use crate::ui::window::move_backward;
use crate::ui::window::move_forward;
use crate::ui::window::paste;
use crate::ui::window::toggle_transparency;
use crate::ZohaCtx;

const ACTION__WIN__QUIT: &str = "quit";
const ACTION__ZOHA__TAB_ADD: &str = "zoha.tab_add";
const ACTION__ZOHA__TAB_CLOSE: &str = "zoha.tab_close";
const ACTION__ZOHA__TAB_MOVE_BACKWARD: &str = "zoha.tab_move_backward";
const ACTION__ZOHA__TAB_MOVE_FORWARD: &str = "zoha.tab_move_forward";
const ACTION__ZOHA__TAB_GOTO_NEXT: &str = "zoha.tab_goto_next";
const ACTION__ZOHA__TAB_GOTO_PREVIOUS: &str = "zoha.tab_goto_previous";
const ACTION__ZOHA__TAB_GOTO_LAST: &str = "zoha.tab_goto_last";
const ACTION__ZOHA__TAB_GOTO_01: &str = "zoha.tab_goto_01";
const ACTION__ZOHA__TAB_GOTO_02: &str = "zoha.tab_goto_02";
const ACTION__ZOHA__TAB_GOTO_03: &str = "zoha.tab_goto_03";
const ACTION__ZOHA__TAB_GOTO_04: &str = "zoha.tab_goto_04";
const ACTION__ZOHA__TAB_GOTO_05: &str = "zoha.tab_goto_05";
const ACTION__ZOHA__TAB_GOTO_06: &str = "zoha.tab_goto_06";
const ACTION__ZOHA__TAB_GOTO_07: &str = "zoha.tab_goto_07";
const ACTION__ZOHA__TAB_GOTO_08: &str = "zoha.tab_goto_08";
const ACTION__ZOHA__COPY: &str = "zoha.copy";
const ACTION__ZOHA__PASTE: &str = "zoha.paste";
const ACTION__ZOHA__TRANSPARENCY_TOGGLE: &str = "zoha.transparency";
const ACTION__ZOHA__FONT_INC: &str = "zoha.font_inc";
const ACTION__ZOHA__FONT_DEC: &str = "zoha.font_dec";
const ACTION__ZOHA__FONT_RESET: &str = "zoha.font_reset";

pub fn set_app_actions(ctx: &ZohaCtx,
                       application: &Application) {
    if let Some(quit) = &ctx.cfg.keys.quit {
        application.set_accels_for_action(
            &format!("win.{}", ACTION__WIN__QUIT),
            &[&quit],
        );
    }

    if let Some(tab_add) = &ctx.cfg.keys.tab_add {
        application.set_accels_for_action(
            &format!("win.{}", ACTION__ZOHA__TAB_ADD),
            &[&tab_add],
        );
    }

    if let Some(tab_close) = &ctx.cfg.keys.tab_close {
        application.set_accels_for_action(
            &format!("win.{}", ACTION__ZOHA__TAB_CLOSE),
            &[&tab_close],
        );
    }

    if let Some(tab_move_fwd) = &ctx.cfg.keys.tab_move_forward {
        application.set_accels_for_action(
            &format!("win.{}", ACTION__ZOHA__TAB_MOVE_FORWARD),
            &[&tab_move_fwd],
        );
    }

    if let Some(tab_move_bkw) = &ctx.cfg.keys.tab_move_backward {
        application.set_accels_for_action(
            &format!("win.{}", ACTION__ZOHA__TAB_MOVE_BACKWARD),
            &[&tab_move_bkw],
        );
    }

    if let Some(tab_goto_next) = &ctx.cfg.keys.tab_goto_next {
        application.set_accels_for_action(
            &format!("win.{}", ACTION__ZOHA__TAB_GOTO_NEXT),
            &[&tab_goto_next],
        );
    }

    if let Some(tab_goto_previous) = &ctx.cfg.keys.tab_goto_previous {
        application.set_accels_for_action(
            &format!("win.{}", ACTION__ZOHA__TAB_GOTO_PREVIOUS),
            &[&tab_goto_previous],
        );
    }

    if let Some(tab_goto_last) = &ctx.cfg.keys.tab_goto_last {
        application.set_accels_for_action(
            &format!("win.{}", ACTION__ZOHA__TAB_GOTO_LAST),
            &[&tab_goto_last],
        );
    }

    if let Some(tab_goto_01) = &ctx.cfg.keys.tab_goto_01 {
        application.set_accels_for_action(
            &format!("win.{}", ACTION__ZOHA__TAB_GOTO_01),
            &[&tab_goto_01],
        );
    }

    if let Some(tab_goto_02) = &ctx.cfg.keys.tab_goto_02 {
        application.set_accels_for_action(
            &format!("win.{}", ACTION__ZOHA__TAB_GOTO_02),
            &[&tab_goto_02],
        );
    }

    if let Some(tab_goto_03) = &ctx.cfg.keys.tab_goto_03 {
        application.set_accels_for_action(
            &format!("win.{}", ACTION__ZOHA__TAB_GOTO_03),
            &[&tab_goto_03],
        );
    }

    if let Some(tab_goto_04) = &ctx.cfg.keys.tab_goto_04 {
        application.set_accels_for_action(
            &format!("win.{}", ACTION__ZOHA__TAB_GOTO_04),
            &[&tab_goto_04],
        );
    }

    if let Some(tab_goto_05) = &ctx.cfg.keys.tab_goto_05 {
        application.set_accels_for_action(
            &format!("win.{}", ACTION__ZOHA__TAB_GOTO_05),
            &[&tab_goto_05],
        );
    }

    if let Some(tab_goto_06) = &ctx.cfg.keys.tab_goto_06 {
        application.set_accels_for_action(
            &format!("win.{}", ACTION__ZOHA__TAB_GOTO_06),
            &[&tab_goto_06],
        );
    }

    if let Some(tab_goto_07) = &ctx.cfg.keys.tab_goto_07 {
        application.set_accels_for_action(
            &format!("win.{}", ACTION__ZOHA__TAB_GOTO_07),
            &[&tab_goto_07],
        );
    }

    if let Some(tab_goto_08) = &ctx.cfg.keys.tab_goto_08 {
        application.set_accels_for_action(
            &format!("win.{}", ACTION__ZOHA__TAB_GOTO_08),
            &[&tab_goto_08],
        );
    }

    // ---------------------------------

    if let Some(copy) = &ctx.cfg.keys.copy {
        application.set_accels_for_action(
            &format!("win.{}", ACTION__ZOHA__COPY),
            &[&copy],
        );
    }

    if let Some(paste) = &ctx.cfg.keys.paste {
        application.set_accels_for_action(
            &format!("win.{}", ACTION__ZOHA__PASTE),
            &[&paste],
        );
    }

    // ---------------------------------

    if let Some(transparency_toggle) = &ctx.cfg.keys.transparency_toggle {
        application.set_accels_for_action(
            &format!("win.{}", ACTION__ZOHA__TRANSPARENCY_TOGGLE),
            &[&transparency_toggle],
        );
    }

    // ---------------------------------

    if let Some(font_inc) = &ctx.cfg.keys.font_size_inc {
        application.set_accels_for_action(
            &format!("win.{}", ACTION__ZOHA__FONT_INC),
            &[&font_inc],
        );
    }

    if let Some(font_dec) = &ctx.cfg.keys.font_size_dec {
        application.set_accels_for_action(
            &format!("win.{}", ACTION__ZOHA__FONT_DEC),
            &[&font_dec],
        );
    }

    if let Some(font_reset) = &ctx.cfg.keys.font_size_reset {
        application.set_accels_for_action(
            &format!("win.{}", ACTION__ZOHA__FONT_RESET),
            &[&font_reset],
        );
    }
}

pub fn set_win_actions(ctx: &Rc<RefCell<ZohaCtx>>) {
    let cxb = ctx.borrow();

    let window: &ApplicationWindow = cxb.get_window().unwrap();
    window.set_show_menubar(false);

    if let Some(key) = cxb.cfg.keys.quit.as_ref() {
        let action = SimpleAction::new(ACTION__WIN__QUIT, None);
        action.connect_activate(clone!(@weak window => move |_,_| {
            window.close();
        }));
        window.add_action(&action);

        debug!("set quit key to: {}", key);
    } else {
        debug!("quit key not set");
    }

    // ---------------------------------

    if let Some(key) = cxb.cfg.keys.tab_add.as_ref() {
        let sa = SimpleAction::new(ACTION__ZOHA__TAB_ADD, None);
        let ctx = Rc::clone(ctx);
        sa.connect_activate(move |_, _| {
            add_tab(&ctx, true);
        });
        window.add_action(&Action::from(sa));

        debug!("set tab_add key to: {}", key);
    } else {
        debug!("tab_add key not set");
    }

    if let Some(key) = cxb.cfg.keys.tab_close.as_ref() {
        let sa = SimpleAction::new(ACTION__ZOHA__TAB_CLOSE, None);
        let ctx = Rc::clone(ctx);
        sa.connect_activate(move |_, _| {
            close_tab(&ctx);
        });
        window.add_action(&Action::from(sa));

        debug!("set tab_close key to: {}", key);
    } else {
        debug!("tab_close key not set");
    }

    if let Some(key) = cxb.cfg.keys.tab_move_backward.as_ref() {
        let sa = SimpleAction::new(ACTION__ZOHA__TAB_MOVE_BACKWARD, None);
        let ctx = Rc::clone(ctx);
        sa.connect_activate(move |_, _| {
            move_backward(&ctx);
        });
        window.add_action(&Action::from(sa));

        debug!("set tab_move_backward key to: {}", key);
    } else {
        debug!("tab_move_backward key not set");
    }

    if let Some(key) = cxb.cfg.keys.tab_move_forward.as_ref() {
        let sa = SimpleAction::new(ACTION__ZOHA__TAB_MOVE_FORWARD, None);
        let ctx = Rc::clone(ctx);
        sa.connect_activate(move |_, _| {
            move_forward(&ctx);
        });
        window.add_action(&Action::from(sa));

        debug!("set tab_move_forward key to: {}", key);
    } else {
        debug!("tab_move_forward key not set");
    }

    if let Some(key) = cxb.cfg.keys.tab_goto_next.as_ref() {
        let sa = SimpleAction::new(ACTION__ZOHA__TAB_GOTO_NEXT, None);
        let ctx = Rc::clone(ctx);
        sa.connect_activate(move |_, _| {
            goto_next(&ctx);
        });
        window.add_action(&Action::from(sa));

        debug!("set tab_goto_next key to: {}", key);
    } else {
        debug!("tab_goto_next key not set");
    }

    if let Some(key) = cxb.cfg.keys.tab_goto_previous.as_ref() {
        let sa = SimpleAction::new(ACTION__ZOHA__TAB_GOTO_PREVIOUS, None);
        let ctx = Rc::clone(ctx);
        sa.connect_activate(move |_, _| {
            goto_previous(&ctx);
        });
        window.add_action(&Action::from(sa));

        debug!("set tab_goto_previous key to: {}", key);
    } else {
        debug!("tab_goto_previous key not set");
    }

    if let Some(key) = cxb.cfg.keys.tab_goto_last.as_ref() {
        let sa = SimpleAction::new(ACTION__ZOHA__TAB_GOTO_LAST, None);
        let ctx = Rc::clone(ctx);
        sa.connect_activate(move |_, _| {
            goto_last(&ctx);
        });
        window.add_action(&Action::from(sa));

        debug!("set tab_goto_last key to: {}", key);
    } else {
        debug!("tab_goto_last key not set");
    }

    if let Some(key) = cxb.cfg.keys.tab_goto_01.as_ref() {
        let sa = SimpleAction::new(ACTION__ZOHA__TAB_GOTO_01, None);
        let ctx = Rc::clone(ctx);
        sa.connect_activate(move |_, _| {
            goto_n(&ctx, 1);
        });
        window.add_action(&Action::from(sa));

        debug!("set tab_goto_01 key to: {}", key);
    } else {
        debug!("tab_goto_01 key not set");
    }

    if let Some(key) = cxb.cfg.keys.tab_goto_02.as_ref() {
        let sa = SimpleAction::new(ACTION__ZOHA__TAB_GOTO_02, None);
        let ctx = Rc::clone(ctx);
        sa.connect_activate(move |_, _| {
            goto_n(&ctx, 2);
        });
        window.add_action(&Action::from(sa));

        debug!("set tab_goto_02 key to: {}", key);
    } else {
        debug!("tab_goto_02 key not set");
    }

    if let Some(key) = cxb.cfg.keys.tab_goto_03.as_ref() {
        let sa = SimpleAction::new(ACTION__ZOHA__TAB_GOTO_03, None);
        let ctx = Rc::clone(ctx);
        sa.connect_activate(move |_, _| {
            goto_n(&ctx, 3);
        });
        window.add_action(&Action::from(sa));

        debug!("set tab_goto_03 key to: {}", key);
    } else {
        debug!("tab_goto_03 key not set");
    }

    if let Some(key) = cxb.cfg.keys.tab_goto_04.as_ref() {
        let sa = SimpleAction::new(ACTION__ZOHA__TAB_GOTO_04, None);
        let ctx = Rc::clone(ctx);
        sa.connect_activate(move |_, _| {
            goto_n(&ctx, 4);
        });
        window.add_action(&Action::from(sa));

        debug!("set tab_goto_04 key to: {}", key);
    } else {
        debug!("tab_goto_04 key not set");
    }

    if let Some(key) = cxb.cfg.keys.tab_goto_05.as_ref() {
        let sa = SimpleAction::new(ACTION__ZOHA__TAB_GOTO_05, None);
        let ctx = Rc::clone(ctx);
        sa.connect_activate(move |_, _| {
            goto_n(&ctx, 5);
        });
        window.add_action(&Action::from(sa));

        debug!("set tab_goto_05 key to: {}", key);
    } else {
        debug!("tab_goto_05 key not set");
    }

    if let Some(key) = cxb.cfg.keys.tab_goto_06.as_ref() {
        let sa = SimpleAction::new(ACTION__ZOHA__TAB_GOTO_06, None);
        let ctx = Rc::clone(ctx);
        sa.connect_activate(move |_, _| {
            goto_n(&ctx, 6);
        });
        window.add_action(&Action::from(sa));

        debug!("set tab_goto_06 key to: {}", key);
    } else {
        debug!("tab_goto_06 key not set");
    }

    if let Some(key) = cxb.cfg.keys.tab_goto_07.as_ref() {
        let sa = SimpleAction::new(ACTION__ZOHA__TAB_GOTO_07, None);
        let ctx = Rc::clone(ctx);
        sa.connect_activate(move |_, _| {
            goto_n(&ctx, 7);
        });
        window.add_action(&Action::from(sa));

        debug!("set tab_goto_07 key to: {}", key);
    } else {
        debug!("tab_goto_07 key not set");
    }

    if let Some(key) = cxb.cfg.keys.tab_goto_08.as_ref() {
        let sa = SimpleAction::new(ACTION__ZOHA__TAB_GOTO_08, None);
        let ctx = Rc::clone(ctx);
        sa.connect_activate(move |_, _| {
            goto_n(&ctx, 8);
        });
        window.add_action(&Action::from(sa));

        debug!("set tab_goto_08 key to: {}", key);
    } else {
        debug!("tab_goto_08 key not set");
    }

    // ---------------------------------

    if let Some(key) = cxb.cfg.keys.copy.as_ref() {
        let sa = SimpleAction::new(ACTION__ZOHA__COPY, None);
        let ctx = Rc::clone(ctx);
        sa.connect_activate(move |_, _| {
            copy(&ctx);
        });
        window.add_action(&Action::from(sa));

        debug!("set copy key to: {}", key);
    } else {
        debug!("copy key not set");
    }

    if let Some(key) = cxb.cfg.keys.paste.as_ref() {
        let sa = SimpleAction::new(ACTION__ZOHA__PASTE, None);
        let ctx = Rc::clone(ctx);
        sa.connect_activate(move |_, _| {
            paste(&ctx);
        });
        window.add_action(&Action::from(sa));

        debug!("set paste key to: {}", key);
    } else {
        debug!("paste key not set");
    }

    // ---------------------------------

    if let Some(key) = cxb.cfg.keys.transparency_toggle.as_ref() {
        let sa =
            SimpleAction::new(ACTION__ZOHA__TRANSPARENCY_TOGGLE, None);
        let ctx = Rc::clone(ctx);
        sa.connect_activate(move |_, _| {
            toggle_transparency(&ctx);
        });
        window.add_action(&Action::from(sa));

        debug!("set transparency_toggle key to: {}", key);
    } else {
        debug!("transparency_toggle key not set");
    }

    // ---------------------------------

    if let Some(key) = cxb.cfg.keys.font_size_inc.as_ref() {
        let sa = SimpleAction::new(ACTION__ZOHA__FONT_INC, None);
        let ctx = Rc::clone(ctx);
        sa.connect_activate(move |_, _| {
            font_inc(&ctx);
        });
        window.add_action(&Action::from(sa));

        debug!("set font_size_inc key to: {}", key);
    } else {
        debug!("font_size_inc key not set");
    }

    if let Some(key) = cxb.cfg.keys.font_size_dec.as_ref() {
        let sa = SimpleAction::new(ACTION__ZOHA__FONT_DEC, None);
        let ctx = Rc::clone(ctx);
        sa.connect_activate(move |_, _| {
            font_dec(&ctx);
        });
        window.add_action(&Action::from(sa));

        debug!("set font_size_dec key to: {}", key);
    } else {
        debug!("font_size_dec key not set");
    }

    if let Some(key) = cxb.cfg.keys.font_size_reset.as_ref() {
        let sa = SimpleAction::new(ACTION__ZOHA__FONT_RESET, None);
        let ctx = Rc::clone(ctx);
        sa.connect_activate(move |_, _| {
            font_reset(&ctx);
        });
        window.add_action(&Action::from(sa));

        debug!("set font_size_reset key to: {}", key);
    } else {
        debug!("font_size_reset key not set");
    }
}
