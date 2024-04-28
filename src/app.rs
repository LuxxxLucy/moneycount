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
    l2r_rate: f64,
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
    Update(String, Column),
    UpdateEntry(usize, Column, String),
    UpdateRate(String),
    NoOp,
}

impl Model {
    fn get_entry_count(&self) -> usize {
        return self.entries.len();
    }

    fn get_sum_one_column(&self, col: Column) -> f64 {
        self.entries
            .iter()
            .filter(|entry| entry.column == col)
            .filter_map(|entry| entry.description.parse::<f64>().ok())
            .sum()
    }

    fn get_sum_left(&self) -> f64 {
        self.get_sum_one_column(Column::Left)
    }

    fn get_sum_right(&self) -> f64 {
        self.get_sum_one_column(Column::Right)
    }
}

impl Application<Msg> for Model {
    fn update(&mut self, msg: Msg) -> Cmd<Self, Msg> {
        match msg {
            Msg::Add => {
                self.entries
                    .push(Entry::new(&self.value, self.column.clone(), self.uid));
                self.uid += 1;
                self.value = "".to_string()
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
            Msg::UpdateRate(rate) => {
                self.l2r_rate = rate.parse::<f64>().unwrap_or(5.35);
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
                font_family: "Roboto, Fira Sans, sans-serif",
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
            l2r_rate: 5.35,
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
        header([class("header")], [h1([], [text("Dual Count")])])
    }

    fn view_input(&self) -> Node<Msg> {
        div(
            [class("entry-row")],
            [
                input(
                    [
                        class("new-item"),
                        id("new-item"),
                        placeholder("CAD"),
                        autofocus(true),
                        value(match self.column {
                            Column::Left => self.value.to_string(),
                            Column::Right => "".to_string(),
                        }),
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
                        placeholder("RMB"),
                        autofocus(true),
                        value(match self.column {
                            Column::Right => self.value.to_string(),
                            Column::Left => "".to_string(),
                        }),
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

        let v = entry.description.parse::<f64>().unwrap_or(0.0);
        let (left_value, right_value) = match entry.column {
            Column::Left => (v, v * self.l2r_rate),
            Column::Right => (v / self.l2r_rate, v),
        };
        li(
            [
                class("item"),
                classes_flag([("editing", entry.editing)]),
                key(format!("item-{}", entry.id)),
            ],
            [div(
                [class("view")],
                [div(
                    [class("entry-row")],
                    [
                        div(
                            [class("entry")],
                            [input(
                                [
                                    class("entry"),
                                    value(format!("{:.2}", left_value)),
                                    // value(format!("{}", left_value)),
                                    on_input(move |input: InputEvent| {
                                        Msg::UpdateEntry(entry_id, Column::Left, input.value())
                                    }),
                                    // on_blur(move |_| Msg::EditingEntry(entry_id, false)),
                                    // on_keypress(move |event: KeyboardEvent| {
                                    //     if event.key_code() == 13 {
                                    //         Msg::EditingEntry(entry_id, false)
                                    //     } else {
                                    //         Msg::NoOp
                                    //     }
                                    // }),
                                ],
                                [],
                            )],
                        ),
                        div(
                            [class("entry")],
                            [input(
                                [
                                    class("entry"),
                                    value(format!("{:.2}", right_value)),
                                    on_input(move |input: InputEvent| {
                                        Msg::UpdateEntry(entry_id, Column::Right, input.value())
                                    }),
                                    // on_blur(move |_| Msg::EditingEntry(entry_id, false)),
                                    // on_keypress(move |event: KeyboardEvent| {
                                    //     if event.key_code() == 13 {
                                    //         Msg::EditingEntry(entry_id, false)
                                    //     } else {
                                    //         Msg::NoOp
                                    //     }
                                    // }),
                                ],
                                [],
                            )],
                        ),
                    ],
                )],
            )],
        )
    }

    fn view_info(&self) -> Node<Msg> {
        let entries_count = self.get_entry_count();

        let left_sum = self.get_sum_left();
        let right_sum = self.get_sum_right();
        let left_total = left_sum + right_sum / self.l2r_rate;
        let right_total = left_sum * self.l2r_rate + right_sum;
        footer(
            [class("footer")],
            [span(
                [class("result")],
                [
                    text!(" {} expenses", entries_count),
                    text!(" CAD {:.2} ", left_total),
                    text!(" RMB {:.2} ", right_total),
                    text!(" under rate "),
                    input(
                        [
                            class("rate"),
                            id("rate"),
                            placeholder("CAD"),
                            autofocus(true),
                            value(self.l2r_rate.clone()),
                            on_input(|v: InputEvent| Msg::UpdateRate(v.value())),
                        ],
                        [],
                    ),
                ],
            )],
        )
    }

    fn info_footer(&self) -> Node<Msg> {
        footer(
            [class("info")],
            [
                p([], [text("A simple two column spreadsheet for writing expenses in two currencies (CAD and RMB) side by side.")]),
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
                            [text("sauron.")],
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
