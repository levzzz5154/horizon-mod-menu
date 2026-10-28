use serde::Deserialize;

pub const PAGE_SIZE: usize = 20;

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct InstalledMod {
    pub id: String,
    pub name: String,
    pub version: String,
    pub kind: String,
    pub enabled: bool,
    #[serde(default)]
    pub problem: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Inventory {
    pub schema: u32,
    pub mods: Vec<InstalledMod>,
}

pub fn parse(bytes: &[u8]) -> Result<Inventory, String> {
    if bytes.len() > 1 << 20 {
        return Err("Inventory is too large; update your launcher.".into());
    }
    let mut inventory: Inventory = serde_json::from_slice(bytes)
        .map_err(|_| "Inventory could not be read; reopen the game through the launcher.")?;
    if inventory.schema != 1 {
        return Err("Inventory needs a newer Mod Menu.".into());
    }
    inventory
        .mods
        .sort_by_key(|m| (m.name.to_lowercase(), m.id.clone()));
    Ok(inventory)
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum Filter {
    #[default]
    All,
    Enabled,
    Disabled,
}
impl Filter {
    pub fn next(self) -> Self {
        match self {
            Self::All => Self::Enabled,
            Self::Enabled => Self::Disabled,
            Self::Disabled => Self::All,
        }
    }
    pub fn label(self) -> &'static str {
        match self {
            Self::All => "All mods",
            Self::Enabled => "Enabled",
            Self::Disabled => "Disabled",
        }
    }
}

pub fn matching<'a>(
    mods: &'a [InstalledMod],
    filter: Filter,
    search: &str,
) -> Vec<&'a InstalledMod> {
    let query = search.to_lowercase();
    mods.iter()
        .filter(|m| {
            let enabled = match filter {
                Filter::All => true,
                Filter::Enabled => m.enabled,
                Filter::Disabled => !m.enabled,
            };
            enabled
                && format!("{} {} {}", m.id, m.name, m.kind)
                    .to_lowercase()
                    .contains(&query)
        })
        .collect()
}

pub fn page_count(count: usize) -> usize {
    count.div_ceil(PAGE_SIZE).max(1)
}

pub fn clean(text: &str) -> String {
    text.chars()
        .filter(|c| !c.is_control() && *c != '§')
        .take(180)
        .collect()
}

impl InstalledMod {
    pub fn row(&self) -> String {
        format!(
            "{} {} | {} | {}",
            clean(&self.name),
            clean(&self.version),
            clean(&self.kind),
            if self.problem.is_some() {
                "Invalid"
            } else if self.enabled {
                "Enabled"
            } else {
                "Disabled"
            }
        )
    }
}
