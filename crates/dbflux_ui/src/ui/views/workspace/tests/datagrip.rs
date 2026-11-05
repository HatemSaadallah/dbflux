use super::{FocusTarget, new_workspace};
use dbflux_ui_sidebar::SidebarTab;
use gpui::TestAppContext;

#[gpui::test]
fn database_explorer_shortcut_selects_expands_and_focuses_the_sidebar(cx: &mut TestAppContext) {
    let (workspace, _, window) = new_workspace(cx);
    window.update(|window, cx| {
        workspace.update(cx, |workspace, cx| {
            workspace.sidebar.update(cx, |sidebar, cx| {
                sidebar.set_active_tab(SidebarTab::Scripts, cx)
            });
            workspace.sidebar_dock.update(cx, |dock, cx| {
                if !dock.is_collapsed() {
                    dock.toggle(cx);
                }
            });
            workspace.set_focus(FocusTarget::Document, window, cx);
        });
    });
    window.simulate_keystrokes("alt-1");
    window.run_until_parked();
    window.update(|_, cx| {
        let workspace = workspace.read(cx);
        assert_eq!(
            workspace.sidebar.read(cx).active_tab(),
            SidebarTab::Connections
        );
        assert!(!workspace.sidebar_dock.read(cx).is_collapsed());
        assert_eq!(workspace.focus_target, FocusTarget::Sidebar);
    });
}

#[gpui::test]
fn go_to_table_opens_the_palette_from_the_keyboard(cx: &mut TestAppContext) {
    let (workspace, _, window) = new_workspace(cx);
    window.simulate_keystrokes("ctrl-n");
    assert!(window.update(|_, cx| workspace.read(cx).command_palette.read(cx).is_visible()));
    window.simulate_keystrokes("escape");
    assert!(!window.update(|_, cx| workspace.read(cx).command_palette.read(cx).is_visible()));
}
