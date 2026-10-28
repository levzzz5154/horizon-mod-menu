pub mod model;
use horizon_mod_sdk::{
    Mod, Registration, input, storage,
    ui::{self, ButtonWidget, ListWidget, Screen, TextWidget, Widget},
};
use model::{Filter, InstalledMod, PAGE_SIZE};

const LIST: u32 = 1;
const FILTER: u32 = 2;
const PREVIOUS: u32 = 3;
const NEXT: u32 = 4;
const REFRESH: u32 = 5;
const CLEAR: u32 = 6;

#[derive(Default)]
struct ModMenu {
    mods: Vec<InstalledMod>,
    error: Option<String>,
    filter: Filter,
    query: String,
    page: usize,
    selected: Option<usize>,
    settings: bool,
}

fn text(id: u32, value: impl Into<String>) -> Widget {
    Widget::Text(TextWidget {
        id,
        text: value.into(),
        color: 0,
    })
}
fn button(id: u32, label: impl Into<String>) -> Widget {
    Widget::Button(ButtonWidget {
        id,
        label: label.into(),
    })
}

impl ModMenu {
    fn read(&mut self) {
        let result = storage::get("inventory.json")
            .ok_or_else(|| {
                "No inventory feed. Update the launcher, then reopen the game.".to_owned()
            })
            .and_then(|b| model::parse(&b));
        match result {
            Ok(inventory) => {
                self.mods = inventory.mods;
                self.error = None;
            }
            Err(error) => {
                self.mods.clear();
                self.error = Some(error);
            }
        }
        self.page = 0;
        self.selected = None;
    }
    fn show(&self) {
        let rows = model::matching(&self.mods, self.filter, &self.query);
        let page = self.page.min(model::page_count(rows.len()) - 1);
        let mut widgets = vec![
            text(
                10,
                format!(
                    "{} installed | {} enabled",
                    self.mods.len(),
                    self.mods.iter().filter(|m| m.enabled).count()
                ),
            ),
            text(
                11,
                "Installed state from your launcher; changes apply on the next launch.",
            ),
        ];
        if let Some(error) = &self.error {
            widgets.push(text(12, error));
        }
        widgets.push(button(FILTER, format!("Show: {}", self.filter.label())));
        if !self.query.is_empty() {
            widgets.push(button(
                CLEAR,
                format!("Clear search: {}", model::clean(&self.query)),
            ));
        }
        widgets.push(Widget::List(ListWidget {
            id: LIST,
            rows: rows
                .iter()
                .skip(page * PAGE_SIZE)
                .take(PAGE_SIZE)
                .map(|m| m.row())
                .collect(),
            selected: self
                .selected
                .filter(|&i| i < rows.len().saturating_sub(page * PAGE_SIZE).min(PAGE_SIZE))
                .map(|i| i as u32),
        }));
        if rows.is_empty() && self.error.is_none() {
            widgets.push(text(13, "No matching installed mods."));
        }
        if let Some(m) = self.selected.and_then(|i| rows.get(page * PAGE_SIZE + i)) {
            widgets.push(text(
                14,
                format!(
                    "ID: {} | Type: {}",
                    model::clean(&m.id),
                    model::clean(&m.kind)
                ),
            ));
            if let Some(problem) = &m.problem {
                widgets.push(text(15, model::clean(problem)));
            }
        }
        widgets.push(text(
            16,
            format!("Page {} / {}", page + 1, model::page_count(rows.len())),
        ));
        if page > 0 {
            widgets.push(button(PREVIOUS, "Previous page"));
        }
        if page + 1 < model::page_count(rows.len()) {
            widgets.push(button(NEXT, "Next page"));
        }
        widgets.push(button(REFRESH, "Refresh installed mods"));
        if self.settings {
            ui::set_settings(&widgets);
        } else {
            ui::open_screen(&Screen {
                title: "Mod Menu".into(),
                widgets,
            });
        }
    }
}

impl Mod for ModMenu {
    fn init() -> Result<(Self, Registration), String> {
        input::register_key("open", "Open Mod Menu", "key.keyboard.m")?;
        let mut menu = Self::default();
        menu.read();
        menu.settings = true;
        menu.show();
        menu.settings = false;
        Ok((menu, Registration::new().command("modmenu").on_tick()))
    }
    fn tick(&mut self, _tick: u64) {
        if input::poll()
            .iter()
            .any(|e| matches!(e, input::Event::KeyDown(name) if name == "open"))
        {
            self.settings = false;
            self.read();
            self.show();
        }
    }
    fn command(&mut self, _name: &str, args: &str) -> Result<Option<String>, String> {
        self.settings = false;
        self.query = args.trim().to_owned();
        self.read();
        self.show();
        Ok(None)
    }
    fn ui_settings_opened(&mut self) {
        self.settings = true;
        self.read();
        self.show();
    }
    fn ui_select(&mut self, id: u32, row: u32) {
        match id {
            LIST => self.selected = Some(row as usize),
            FILTER => {
                self.filter = self.filter.next();
                self.page = 0;
                self.selected = None;
            }
            PREVIOUS => {
                self.page = self.page.saturating_sub(1);
                self.selected = None;
            }
            NEXT => {
                self.page = (self.page + 1).min(
                    model::page_count(model::matching(&self.mods, self.filter, &self.query).len())
                        - 1,
                );
                self.selected = None;
            }
            REFRESH => self.read(),
            CLEAR => {
                self.query.clear();
                self.page = 0;
                self.selected = None;
            }
            _ => return,
        }
        self.show();
    }
}

horizon_mod_sdk::export_mod!(ModMenu);
