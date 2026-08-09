use std::borrow::Cow;

use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Paragraph, Wrap},
    Frame,
};

use super::release_notes::release_notes_close_button_rect;
use super::scrollbar::{release_notes_scrollbar_rect, render_scrollbar};
use super::widgets::{
    modal_stack_areas, panel_contrast_fg, render_action_button, render_modal_header,
    render_modal_shell,
};
use crate::app::AppState;

pub(super) type HelpEntry = (String, Cow<'static, str>);
pub(super) type HelpGroup = (&'static str, Vec<HelpEntry>);

fn help_entry(key: impl Into<String>, label: &'static str) -> HelpEntry {
    (key.into(), Cow::Borrowed(label))
}

fn keybind_label(bindings: &crate::config::ActionKeybinds) -> String {
    bindings.label().unwrap_or_else(|| "未设置".to_string())
}

fn indexed_label(bindings: &[crate::config::IndexedKeybind]) -> String {
    if bindings.is_empty() {
        return "未设置".to_string();
    }

    let mut parts = Vec::new();
    let mut index = 0;
    while index < bindings.len() {
        if let Some(prefix) = indexed_range_prefix(&bindings[index..]) {
            parts.push(format!("{prefix}1..9"));
            index += 9;
        } else {
            parts.push(bindings[index].label.clone());
            index += 1;
        }
    }

    parts.join(" / ")
}

fn indexed_range_prefix(bindings: &[crate::config::IndexedKeybind]) -> Option<&str> {
    let run = bindings.get(..9)?;
    let prefix = run[0].label.strip_suffix('1')?;
    for (offset, binding) in run.iter().enumerate() {
        let digit = char::from(b'1' + offset as u8);
        if binding.label.strip_suffix(digit) != Some(prefix) {
            return None;
        }
    }
    Some(prefix)
}

pub(super) fn keybind_help_groups(app: &AppState) -> Vec<HelpGroup> {
    let kb = &app.keybinds;
    let mut groups = Vec::new();

    groups.push((
        "全局",
        vec![
            help_entry(
                crate::config::format_key_combo((app.prefix_code, app.prefix_mods)),
                "前缀键模式",
            ),
            help_entry(keybind_label(&kb.help), "快捷键"),
            help_entry(keybind_label(&kb.settings), "设置"),
            help_entry(keybind_label(&kb.detach), "分离"),
            help_entry(keybind_label(&kb.reload_config), "重新加载配置"),
            help_entry(keybind_label(&kb.open_notification_target), "打开通知目标"),
        ],
    ));

    groups.push((
        "导航",
        vec![
            help_entry("esc", "返回"),
            help_entry(
                format!(
                    "{} / {}",
                    keybind_label(&kb.navigate.workspace_up),
                    keybind_label(&kb.navigate.workspace_down)
                ),
                "工作区列表",
            ),
            help_entry(
                format!(
                    "{} / {} / {} / {} / left / right",
                    keybind_label(&kb.navigate.pane_left),
                    keybind_label(&kb.navigate.pane_down),
                    keybind_label(&kb.navigate.pane_up),
                    keybind_label(&kb.navigate.pane_right)
                ),
                "移动焦点",
            ),
            help_entry("tab / shift+tab", "循环切换窗格"),
            help_entry("enter", "打开工作区"),
            help_entry("1..9", "切换工作区"),
        ],
    ));

    let workspace_tab = vec![
        help_entry(keybind_label(&kb.workspace_picker), "工作区导航"),
        help_entry(keybind_label(&kb.goto), "会话导航器"),
        help_entry(keybind_label(&kb.new_workspace), "新建工作区"),
        help_entry(keybind_label(&kb.new_worktree), "新建工作树"),
        help_entry(keybind_label(&kb.open_worktree), "打开工作树"),
        help_entry(keybind_label(&kb.remove_worktree), "删除工作树检出"),
        help_entry(keybind_label(&kb.rename_workspace), "重命名工作区"),
        help_entry(keybind_label(&kb.close_workspace), "关闭工作区"),
        help_entry(keybind_label(&kb.previous_workspace), "上一个工作区"),
        help_entry(keybind_label(&kb.next_workspace), "下一个工作区"),
        help_entry(indexed_label(&kb.switch_workspace), "切换工作区 1-9"),
        help_entry(keybind_label(&kb.previous_agent), "上一个 Agent"),
        help_entry(keybind_label(&kb.next_agent), "下一个 Agent"),
        help_entry(indexed_label(&kb.focus_agent), "聚焦 Agent 1-9"),
        help_entry(keybind_label(&kb.new_tab), "新建标签页"),
        help_entry(keybind_label(&kb.rename_tab), "重命名标签页"),
        help_entry(keybind_label(&kb.previous_tab), "上一个标签页"),
        help_entry(keybind_label(&kb.next_tab), "下一个标签页"),
        help_entry(indexed_label(&kb.switch_tab), "切换标签页 1-9"),
        help_entry(keybind_label(&kb.close_tab), "关闭标签页"),
    ];
    groups.push(("工作区 / 标签页", workspace_tab));

    let panes = vec![
        help_entry(keybind_label(&kb.split_vertical), "垂直分割"),
        help_entry(keybind_label(&kb.split_horizontal), "水平分割"),
        help_entry(keybind_label(&kb.close_pane), "关闭窗格"),
        help_entry(keybind_label(&kb.rename_pane), "重命名窗格"),
        help_entry(keybind_label(&kb.edit_scrollback), "编辑滚动历史"),
        help_entry(keybind_label(&kb.copy_mode), "复制模式"),
        help_entry(keybind_label(&kb.zoom), "缩放窗格"),
        help_entry(keybind_label(&kb.resize_mode), "调整大小模式"),
        help_entry(keybind_label(&kb.toggle_sidebar), "切换侧边栏"),
        help_entry(keybind_label(&kb.focus_pane_left), "聚焦左侧窗格"),
        help_entry(keybind_label(&kb.focus_pane_down), "聚焦下方窗格"),
        help_entry(keybind_label(&kb.focus_pane_up), "聚焦上方窗格"),
        help_entry(keybind_label(&kb.focus_pane_right), "聚焦右侧窗格"),
        help_entry(keybind_label(&kb.cycle_pane_next), "循环切换到下一个窗格"),
        help_entry(
            keybind_label(&kb.cycle_pane_previous),
            "循环切换到上一个窗格",
        ),
        help_entry(keybind_label(&kb.last_pane), "最近使用的窗格"),
    ];
    groups.push(("窗格", panes));

    if !kb.custom_commands.is_empty() {
        groups.push((
            "自定义",
            kb.custom_commands
                .iter()
                .map(|binding| {
                    (
                        binding.label.clone(),
                        binding
                            .description
                            .clone()
                            .map(Cow::Owned)
                            .unwrap_or(Cow::Borrowed("自定义命令")),
                    )
                })
                .collect(),
        ));
    }

    groups
}

fn filter_keybind_help_groups(groups: Vec<HelpGroup>, query: &str) -> Vec<HelpGroup> {
    if query.is_empty() {
        return groups;
    }

    let query = query.to_lowercase();
    groups
        .into_iter()
        .filter_map(|(group, entries)| {
            let entries = entries
                .into_iter()
                .filter(|(key, label)| {
                    key.to_lowercase().contains(&query) || label.to_lowercase().contains(&query)
                })
                .collect::<Vec<_>>();
            (!entries.is_empty()).then_some((group, entries))
        })
        .collect()
}

pub(crate) fn keybind_help_lines(app: &AppState) -> Vec<(usize, Line<'static>)> {
    let heading_style = Style::default()
        .fg(app.palette.accent)
        .add_modifier(Modifier::BOLD);
    let key_style = Style::default()
        .fg(app.palette.mauve)
        .add_modifier(Modifier::BOLD);
    let label_style = Style::default().fg(app.palette.text);

    let groups = filter_keybind_help_groups(keybind_help_groups(app), &app.keybind_help.query);
    let key_width = groups
        .iter()
        .flat_map(|(_, entries)| entries.iter().map(|(key, _)| key.chars().count()))
        .max()
        .unwrap_or(8);

    let mut lines = Vec::new();

    if groups.is_empty() {
        let message = " 没有匹配的快捷键";
        return vec![(
            message.chars().count(),
            Line::from(Span::styled(
                message,
                Style::default().fg(app.palette.overlay1),
            )),
        )];
    }

    for (group, entries) in groups {
        lines.push((
            group.len() + 1,
            Line::from(vec![Span::styled(format!(" {group}"), heading_style)]),
        ));
        for (key, label) in entries {
            let padded_key = format!(" {:<width$} ", key, width = key_width);
            let width = padded_key.chars().count() + label.chars().count();
            lines.push((
                width,
                Line::from(vec![
                    Span::styled(padded_key, key_style),
                    Span::styled(label.into_owned(), label_style),
                ]),
            ));
        }
        lines.push((0, Line::raw("")));
    }

    lines
}

pub(super) fn render_keybind_help_overlay(app: &AppState, frame: &mut Frame) {
    super::dim_background(frame, frame.area());

    let Some(inner) = render_modal_shell(frame, frame.area(), 76, 22, &app.palette) else {
        return;
    };
    if inner.height < 6 || inner.width < 20 {
        return;
    }

    let stack = modal_stack_areas(inner, 2, 1, 0, 1);
    let header_rows =
        Layout::vertical([Constraint::Length(1), Constraint::Length(1)]).areas::<2>(stack.header);

    render_modal_header(frame, header_rows[0], "快捷键", &app.palette);
    render_action_button(
        frame,
        release_notes_close_button_rect(header_rows[0]),
        Some("esc"),
        if app.keybind_help.search_focused {
            "返回"
        } else {
            "关闭"
        },
        Style::default()
            .fg(panel_contrast_fg(&app.palette))
            .bg(app.palette.accent)
            .add_modifier(Modifier::BOLD),
    );
    let search_line = if app.keybind_help.search_focused {
        Line::from(vec![
            Span::styled(
                " / ",
                Style::default()
                    .fg(app.palette.accent)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                app.keybind_help.query.as_str(),
                Style::default()
                    .fg(app.palette.text)
                    .add_modifier(Modifier::BOLD),
            ),
        ])
    } else {
        Line::from(Span::styled(
            " 按 / 可按命令或快捷键筛选",
            Style::default().fg(app.palette.overlay0),
        ))
    };
    frame.render_widget(Paragraph::new(search_line), header_rows[1]);

    let body_area = stack.content;
    let metrics = crate::pane::ScrollMetrics {
        offset_from_bottom: app
            .keybind_help_max_scroll()
            .saturating_sub(app.keybind_help.scroll) as usize,
        max_offset_from_bottom: app.keybind_help_max_scroll() as usize,
        viewport_rows: body_area.height.max(1) as usize,
    };
    let track = release_notes_scrollbar_rect(body_area, metrics);
    let text_area = track
        .map(|_| {
            Rect::new(
                body_area.x,
                body_area.y,
                body_area.width.saturating_sub(1),
                body_area.height,
            )
        })
        .unwrap_or(body_area);

    let body = Paragraph::new(
        keybind_help_lines(app)
            .into_iter()
            .map(|(_, line)| line)
            .collect::<Vec<_>>(),
    )
    .wrap(Wrap { trim: false })
    .scroll((app.keybind_help.scroll, 0));
    frame.render_widget(body, text_area);
    if let Some(track) = track {
        render_scrollbar(
            frame,
            metrics,
            track,
            app.palette.overlay0,
            app.palette.overlay1,
            "▐",
        );
    }

    let footer = if app.keybind_help.search_focused {
        Line::from(vec![
            Span::styled(" 筛选 ", Style::default().fg(app.palette.overlay0)),
            Span::styled("输入/退格", Style::default().fg(app.palette.text)),
            Span::styled(" · ", Style::default().fg(app.palette.overlay0)),
            Span::styled("清除 ", Style::default().fg(app.palette.overlay0)),
            Span::styled("ctrl+u", Style::default().fg(app.palette.text)),
            Span::styled(" · ", Style::default().fg(app.palette.overlay0)),
            Span::styled("滚动 ", Style::default().fg(app.palette.overlay0)),
            Span::styled("↑↓/pgup/pgdn", Style::default().fg(app.palette.text)),
            Span::styled(" · ", Style::default().fg(app.palette.overlay0)),
            Span::styled("返回 ", Style::default().fg(app.palette.overlay0)),
            Span::styled("esc", Style::default().fg(app.palette.text)),
        ])
    } else {
        Line::from(vec![
            Span::styled(" 搜索 ", Style::default().fg(app.palette.overlay0)),
            Span::styled("/", Style::default().fg(app.palette.text)),
            Span::styled(" · ", Style::default().fg(app.palette.overlay0)),
            Span::styled("滚动 ", Style::default().fg(app.palette.overlay0)),
            Span::styled("j/k/↑↓/pgup/pgdn", Style::default().fg(app.palette.text)),
            Span::styled(" · ", Style::default().fg(app.palette.overlay0)),
            Span::styled("关闭 ", Style::default().fg(app.palette.overlay0)),
            Span::styled("esc/enter", Style::default().fg(app.palette.text)),
        ])
    };
    frame.render_widget(Paragraph::new(footer), stack.footer.unwrap_or_default());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn groups() -> Vec<HelpGroup> {
        vec![
            (
                "workspaces / tabs",
                vec![
                    help_entry("w", "workspace navigation"),
                    help_entry("c", "new tab"),
                ],
            ),
            (
                "panes",
                vec![
                    help_entry("v", "split vertical"),
                    help_entry("x", "close pane"),
                ],
            ),
        ]
    }

    #[test]
    fn keybind_help_filter_matches_labels_case_insensitively() {
        let filtered = filter_keybind_help_groups(groups(), "WoRk");

        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].0, "workspaces / tabs");
        assert_eq!(filtered[0].1.len(), 1);
        assert_eq!(filtered[0].1[0].1, "workspace navigation");
    }

    #[test]
    fn keybind_help_filter_matches_shortcuts_without_matching_group_headings() {
        let filtered = filter_keybind_help_groups(groups(), "x");

        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].0, "panes");
        assert_eq!(filtered[0].1.len(), 1);
        assert_eq!(filtered[0].1[0].1, "close pane");

        assert!(filter_keybind_help_groups(groups(), "panes").is_empty());
    }
}
