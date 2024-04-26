use sauron::{
    dom::events::KeyboardEvent,
    html::{attributes::*, events::*, *},
    jss, text, Application, Cmd, Node,
};
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Model {
    entries: Vec<Entry>,
    value: String,
    column: Column,
    uid: usize,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
pub enum Column {
    Left,
    Right,
}

#[derive(Serialize, Deserialize)]
struct Entry {
    description: String,
    column: Column, // indicate left or right
    editing: bool,
    id: usize,
}

pub enum Msg {
    Add,
    EditingEntry(usize, bool),
    Update(String, Column),
    UpdateEntry(usize, Column, String),
    ToggleEdit(usize),
    NoOp,
}

impl Application for Model {
    type MSG = Msg;

    fn update(&mut self, msg: Msg) -> Cmd<Msg> {
        match msg {
            Msg::Add => {
                self.entries
                    .push(Entry::new(&self.value, self.column.clone(), self.uid));
                self.uid += 1;
                self.value = "".to_string()
            }
            Msg::EditingEntry(id, is_editing) => {
                self.entries.iter_mut().for_each(|entry| {
                    if entry.id == id {
                        entry.editing = is_editing;
                    }
                });
            }
            Msg::Update(val, col) => {
                self.value = val;
                self.column = col;
            }
            Msg::UpdateEntry(id, new_col, new_description) => {
                self.entries.iter_mut().for_each(|entry| {
                    if entry.id == id {
                        entry.description = new_description.clone();
                        entry.column = new_col.clone();
                    }
                });
            }
            Msg::ToggleEdit(id) => {
                self.entries.iter_mut().for_each(|entry| {
                    if entry.id == id {
                        entry.editing = !entry.editing;
                    }
                });
            }
            Msg::NoOp => {}
        }
        #[cfg(feature = "with-storage")]
        self.save_to_storage();
        Cmd::none()
    }

    fn view(&self) -> Node<Msg> {
        div(
            [class("countapp-wrapper")],
            [
                section([class("countapp")], [self.view_header()]),
                section(
                    [class("countapp")],
                    [self.view_entries(), self.view_input(), self.view_info()],
                ),
                self.info_footer(),
            ],
        )
    }

    fn stylesheet() -> Vec<String> {
        vec![jss! {
            "body": {
                font_family: "Fira Sans, Courier New, Courier, Lucida Sans Typewriter, Lucida Typewriter, monospace",
            }
        }]
    }
}

impl Entry {
    fn new(description: &str, col: Column, id: usize) -> Self {
        Entry {
            description: description.to_string(),
            column: col,
            editing: false,
            id,
        }
    }
}

impl Model {
    pub(crate) fn new() -> Self {
        Model {
            entries: vec![],
            value: "".into(),
            column: Column::Left,
            uid: 0,
        }
    }

    fn view_entries(&self) -> Node<Msg> {
        section(
            [class("main")],
            [ul([class("item-list")], {
                self.entries.iter().map(|entry| self.view_entry(entry))
            })],
        )
    }

    fn view_header(&self) -> Node<Msg> {
        header([class("header")], [h1([], [text("Counter")])])
    }

    fn view_input(&self) -> Node<Msg> {
        div(
            [class("input-row")],
            [
                input(
                    [
                        class("new-item"),
                        id("new-item"),
                        placeholder("currency 1"),
                        autofocus(true),
                        value(self.value.to_string()),
                        on_input(|v: InputEvent| Msg::Update(v.value(), Column::Left)),
                        on_keypress(|event: KeyboardEvent| {
                            if event.key() == "Enter" {
                                Msg::Add
                            } else {
                                Msg::NoOp
                            }
                        }),
                    ],
                    [],
                ),
                input(
                    [
                        class("new-item"),
                        id("new-item"),
                        placeholder("currency 2"),
                        autofocus(true),
                        value(self.value.to_string()),
                        on_input(|v: InputEvent| Msg::Update(v.value(), Column::Right)),
                        on_keypress(|event: KeyboardEvent| {
                            if event.key() == "Enter" {
                                Msg::Add
                            } else {
                                Msg::NoOp
                            }
                        }),
                    ],
                    [],
                ),
            ],
        )
    }

    fn view_entry(&self, entry: &Entry) -> Node<Msg> {
        let entry_id = entry.id;
        li(
            [
                class("item"),
                classes_flag([("editing", entry.editing)]),
                key(format!("item-{}", entry.id)),
            ],
            [
                div(
                    [class("view")],
                    [label(
                        [on_doubleclick(move |_| Msg::ToggleEdit(entry_id))],
                        [div(
                            [class("entry-row")],
                            [
                                div([class("entry")], [text(entry.description.to_string())]),
                                div(
                                    [class("entry")],
                                    [text(match entry.column {
                                        Column::Left => "right ",
                                        Column::Right => "left ",
                                    })],
                                ),
                            ],
                        )],
                    )],
                ),
                input(
                    [
                        class("edit"),
                        r#type("text"),
                        hidden(!entry.editing),
                        value(&entry.description),
                        on_input(move |input: InputEvent| {
                            Msg::UpdateEntry(entry_id, Column::Left, input.value())
                        }),
                        on_blur(move |_| Msg::EditingEntry(entry_id, false)),
                        on_keypress(move |event: KeyboardEvent| {
                            if event.key_code() == 13 {
                                Msg::EditingEntry(entry_id, false)
                            } else {
                                Msg::NoOp
                            }
                        }),
                    ],
                    [],
                ),
            ],
        )
    }

    fn view_info(&self) -> Node<Msg> {
        let entries_left = self.entries.len();
        let item = if entries_left == 1 { " item" } else { " items" };

        footer(
            [class("footer")],
            [span(
                [class("item-count")],
                [strong([], [text(entries_left)]), text!(" {} left", item)],
            )],
        )
    }

    fn info_footer(&self) -> Node<Msg> {
        footer(
            [class("info")],
            [
                p([], [text("Double-click to edit an entry")]),
                p(
                    [],
                    [
                        text("Written by "),
                        a(
                            [href("http://luxxxlucy.github.io"), target("_blank")],
                            [text("jialin lu LUCY ")],
                        ),
                        text("powered by "),
                        a(
                            [
                                href("https://github.com/ivanceras/sauron"),
                                target("_blank"),
                            ],
                            [text("sauron")],
                        ),
                    ],
                ),
            ],
        )
    }

    #[cfg(feature = "with-storage")]
    fn save_to_storage(&self) {
        let window = web_sys::window().expect("no global `window` exists");
        let local_storage = window.local_storage();
        if let Ok(Some(local_storage)) = local_storage {
            let json_data = serde_json::to_string(&self).expect("must serialize data");
            if let Err(err) = local_storage.set_item("moneycount::data", &json_data) {
                log::error!("Could not write to local storage, {:?}", err);
            }
        }
    }

    #[cfg(feature = "with-storage")]
    pub fn get_from_storage() -> Self {
        let window = web_sys::window().expect("no global `window` exists");
        let local_storage = window.local_storage();

        if let Ok(Some(local_storage)) = local_storage {
            if let Ok(Some(s)) = local_storage.get_item("moneycount::data") {
                serde_json::from_str(&s).ok().unwrap_or(Self::new())
            } else {
                Self::new()
            }
        } else {
            Self::new()
        }
    }
}
