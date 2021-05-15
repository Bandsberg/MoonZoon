<<<<<<< HEAD
// #![no_std]

use zoon::{*, println, raw_el::{attr, tag, event_handler}};
use rand::prelude::*;
use std::rc::Rc;
=======
use zoon::{*, format};
use rand::prelude::*;
use std::{iter::repeat_with, array, ops::Not};
use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};

// ------ ------
//    Statics 
// ------ ------
>>>>>>> a70fd9927f5b90424db36fda687d6c6344278925

static ADJECTIVES: &[&'static str] = &[
    "pretty", "large", "big", "small", "tall", "short", "long", "handsome", "plain", 
    "quaint", "clean", "elegant", "easy", "angry", "crazy", "helpful", "mushy", "odd", 
    "unsightly", "adorable", "important", "inexpensive", "cheap", "expensive", "fancy"
];

static COLOURS: &[&'static str] = &[
    "red", "yellow", "blue", "green", "pink", "brown", "purple", "brown", "white", "black",
    "orange",
];

static NOUNS: &[&'static str] = &[
    "table", "chair", "house", "bbq", "desk", "car", "pony", "cookie", "sandwich", "burger",
    "pizza", "mouse", "keyboard",
];

static NEXT_ID: AtomicUsize = AtomicUsize::new(1);

#[static_ref]
fn selected_row() -> &'static Mutable<Option<ID>> {
    Mutable::new(None)
}

<<<<<<< HEAD
    #[s_var]
    fn generator() -> SVar<SmallRng> {
        SmallRng::from_entropy()
    }

    #[s_var]
    fn previous_id() -> SVar<ID> {
        0
    }
=======
#[static_ref]
fn rows() -> &'static MutableVec<Arc<Row>> {
    MutableVec::new()
}

type ID = usize;
>>>>>>> a70fd9927f5b90424db36fda687d6c6344278925

struct Row {
    id: ID,
    label: Mutable<String>,
}

<<<<<<< HEAD
    #[s_var]
    fn selected_row() -> SVar<Option<VarRef<Row>>> {
        None
    }

    #[s_var]
    fn rows() -> SVar<Vec<Rc<Var<Row>>>> {
        Vec::new()
    }

    #[cache]
    fn rows_len() -> Cache<usize> {
        rows().map(Vec::len)
    }

    fn create_row() -> Rc<Var<Row>> {
        let id = previous_id().map_mut(|id| {
            *id += 1;
            *id
        });
        let label = generator().map_mut(|generator| {
            format!(
                "{} {} {}",
                ADJECTIVES.choose(generator).unwrap(),
                COLOURS.choose(generator).unwrap(),
                NOUNS.choose(generator).unwrap(),
            )
        });
        Rc::new(Var::new(Row { id, label }))
    }

    #[update]
    fn create_rows(count: usize) {
        rows().update_mut(|rows| {
            *rows = (0..count).map(|_| create_row()).collect();
        });
    }

    #[update]
    fn append_rows(count: usize) {
        rows().update_mut(|rows| {
            rows.append(&mut (0..count).map(|_| create_row()).collect());
        });
=======
// ------ ------
//    Signals 
// ------ ------

fn rows_exist() -> impl Signal<Item = bool> {
    rows().signal_vec_cloned().is_empty().map(Not::not)
}

// ------ ------
//   Commands 
// ------ ------

fn create_row() -> Arc<Row> {
    let mut generator = SmallRng::from_entropy();
    let label = format!(
        "{} {} {}",
        ADJECTIVES.choose(&mut generator).unwrap_throw(),
        COLOURS.choose(&mut generator).unwrap_throw(),
        NOUNS.choose(&mut generator).unwrap_throw(),
    );
    Arc::new(Row { 
        id: NEXT_ID.fetch_add(1, Ordering::SeqCst), 
        label: Mutable::new(label)
    })
}

fn create_rows(count: usize) {
    rows()
        .lock_mut()
        .replace_cloned(repeat_with(create_row).take(count).collect())
}

fn append_rows(count: usize) {
    rows()
        .lock_mut()
        .extend(repeat_with(create_row).take(count));
}

fn update_rows(step: usize) {
    let rows = rows().lock_ref();
    for position in (0..rows.len()).step_by(step) {
        rows[position].label.lock_mut().push_str(" !!!");
>>>>>>> a70fd9927f5b90424db36fda687d6c6344278925
    }
}

<<<<<<< HEAD
    #[update]
    fn update_rows(step: usize) {
        let len = rows_len().inner();
        // rows().use_ref(|rows| {
        rows().use_ref(|rows| {
            // stop![
                for position in (0..len).step_by(step) {
                    println!("position: {}", position);
                    rows[position].update_mut(|row| {
                        println!("label: {}", row.label);
                        row.label += " !!!"
                    });
                }
            // ]
        })
    }
=======
fn clear_rows() {
    rows().lock_mut().clear()
}
>>>>>>> a70fd9927f5b90424db36fda687d6c6344278925

fn swap_rows() {
    let mut rows = rows().lock_mut();
    if rows.len() < 999 { return }
    rows.swap(1, 998)
}

fn select_row(id: ID) {
    selected_row().set(Some(id))
}

<<<<<<< HEAD
    #[update]
    fn select_row(row: VarRef<Row>) {
        let old_selected = selected_row().map_mut(|selected_row| {
            selected_row.replace(row)
        });
        // row.mark_updated();
        if let Some(old_selected) = old_selected {
            // old_selected.try_mark_updated();
        }
    }

    #[update]
    fn remove_row(row: VarRef<Row>) {
        rows().update_mut(|rows| {
            let position = rows.iter().position(|r| r.to_var_ref() == row).unwrap();
            rows.remove(position);
        });
    }

    #[cmp]
    fn root() -> Cmp {
        raw_el![
            attr("class", "container"),
=======
fn remove_row(id: ID) {
    rows().lock_mut().retain(|row| row.id != id);
}

// ------ ------
//     View 
// ------ ------

fn root() -> RawEl {
    RawEl::new("div")
        .attr("class", "container")
        .children(array::IntoIter::new([
>>>>>>> a70fd9927f5b90424db36fda687d6c6344278925
            jumbotron(),
            table(),
            RawEl::new("span")
                .attr("class", "preloadicon glyphicon glyphicon-remove")
                .attr("aria-hidden", "")
        ]))
}

<<<<<<< HEAD
    #[cmp]
    fn jumbotron() -> Cmp {
        raw_el![
            attr("class", "jumbotron"),
            raw_el![
                attr("class", "row"),
                raw_el![
                    attr("class", "col-md-6"),
                    raw_el![
                        tag("h1"),
                        "Zoon",
                    ]
                ],
                raw_el![
                    attr("class", "col-md-6"),
                    raw_el![
                        attr("class", "row"),
                        // action_button("run", "Create 1,000 rows", || create_rows(1_000)),
                        action_button("run", "Create 1,000 rows", || create_rows(11)),
                        action_button("runlots", "Create 10,000 rows", || create_rows(10_000)),
                        action_button("add", "Append 1,000 rows", || append_rows(1_000)),
                        action_button("update", "Update every 10th row", || update_rows(10)),
                        action_button("clear", "Clear", clear_rows),
                        action_button("swaprows", "Swap Rows", swap_rows),
                    ]
                ],
            ]
        ]
    }

    fn action_button<'a>(
        id: &'static str, 
        title: &'static str, 
        on_click: fn(),
    ) -> RawEl<'a> {
        raw_el![
            attr("class", "col-sm-6 smallpad"),
            attr("id", id),
            attr("type", "button"),
            event_handler("click", move |_| on_click()),
            title,
        ]
    }

    #[cmp]
    fn table() -> Cmp {
        let rows = rows().map(|rows| {
            rows
                .iter()
                .map(|row_var| row(row_var.to_var_ref()))
                .collect::<Vec<_>>()
        });
        raw_el![
            tag("table"),
            attr("class", "table table-hover table-striped test-data"),
            raw_el![
                tag("tbody"),
                attr("id", "tbody"),
                rows,
            ]
        ]
    }

    #[cmp(row)]
    fn row(row: VarRef<Row>) -> Cmp {
        let selected_row = selected_row()/*.unwatch()*/.inner();
        let is_selected = selected_row == Some(row);
        raw_el![
            tag("tr"),
            is_selected.then(|| attr("class", "danger")),
            row_id(row),
            row_label(row),
            row_remove_button(row),
            raw_el![
                tag("td"),
                attr("class", "col-md-6"),
            ]
        ]
    }

    #[cmp]
    fn row_id(row: VarRef<Row>) -> Cmp {
        let id = row.map(|row| row.id);
        raw_el![
            tag("td"),
            attr("class", "col-md-1"),
            id
        ]
    }

    #[cmp]
    fn row_label(row: VarRef<Row>) -> Cmp {
        let label = row.map(|row| row.label.clone());
        raw_el![
            tag("td"),
            attr("class", "col-md-4"),
            event_handler("click", move |_| select_row(row)),
            raw_el![
                tag("a"),
                attr("class", "lbl"),
                label,
            ]
        ]
    }

    #[cmp]
    fn row_remove_button(row: VarRef<Row>) -> Cmp {
        // row.unwatch();
        raw_el![
            tag("td"),
            attr("class", "col-md-1"),
            raw_el![
                tag("a"),
                attr("class", "remove"),
                event_handler("click", move |_| remove_row(row)),
                raw_el![
                    tag("span"),
                    attr("class", "glyphicon glyphicon-remove remove"),
                    attr("aria-hidden", "true"),
                ]
            ]
        ]
    }
=======
fn jumbotron() -> RawEl {
    RawEl::new("div")
        .attr("class", "jumbotron")
        .child(
            RawEl::new("div")
                .attr("class", "row")
                .children(array::IntoIter::new([
                    RawEl::new("div")
                        .attr("class", "col-md-6")
                        .child(
                            RawEl::new("h1").child("MoonZoon")
                        ),
                    RawEl::new("div")
                        .attr("class", "col-md-6")
                        .child(
                            action_buttons()
                        ),
                ]))
        )
}

fn action_buttons() -> RawEl {
    RawEl::new("div")
        .attr("class", "row")
        .children(array::IntoIter::new([
            action_button("run", "Create 1,000 rows", || create_rows(1_000)),
            action_button("runlots", "Create 10,000 rows", || create_rows(10_000)),
            action_button("add", "Append 1,000 rows", || append_rows(1_000)),
            action_button("update", "Update every 10th row", || update_rows(10)),
            action_button("clear", "Clear", clear_rows),
            action_button("swaprows", "Swap Rows", swap_rows),
        ]))
}

fn action_button(
    id: &'static str, 
    title: &'static str, 
    on_click: fn(),
) -> RawEl {
    RawEl::new("div")
        .attr("class", "col-sm-6 smallpad")
        .child(
            RawEl::new("button")
                .attr("id", id)
                .attr("class", "btn btn-primary btn-block")
                .attr("type", "button")
                .event_handler(move |_: events::Click| on_click())
                .child(
                    title
                )
        )
}

fn table() -> RawEl {
    RawEl::new("table")
        .attr("class", "table table-hover table-striped test-data")
        .child_signal(
            rows_exist().map(|rows_exist| rows_exist.then(|| {
                RawEl::new("tbody")
                    .attr("id", "tbody")
                    .children_signal_vec(
                        rows().signal_vec_cloned().map(row)
                    )
            }))
        )
}

fn row(row: Arc<Row>) -> RawEl {
    let id = row.id;
    RawEl::new("tr")
        .attr_signal(
            "class",
            selected_row().signal_ref(move |selected_id| {
                ((*selected_id)? == id).then(|| "danger")
            })
        )
        .children(array::IntoIter::new([
            row_id(id),
            row_label(id, row.label.signal_cloned()),
            row_remove_button(id),
            RawEl::new("td")
                .attr("class", "col-md-6")
        ]))
}

fn row_id(id: ID) -> RawEl {
    RawEl::new("td")
        .attr("class", "col-md-1")
        .child(id)
}

fn row_label(id: ID, label: impl Signal<Item = String> + Unpin + 'static) -> RawEl {
    RawEl::new("td")
        .attr("class", "col-md-4")
        .child(
            RawEl::new("a")
                .event_handler(move |_: events::Click| select_row(id))
                .child(Text::with_signal(label))
        )
}
>>>>>>> a70fd9927f5b90424db36fda687d6c6344278925

fn row_remove_button(id: ID) -> RawEl {
    RawEl::new("td")
        .attr("class", "col-md-1")
        .child(
            RawEl::new("a")
                .event_handler(move |_: events::Click| remove_row(id))
                .child(
                    RawEl::new("span")
                        .attr("class", "glyphicon glyphicon-remove remove")
                        .attr("aria-hidden", "true"),
                )
        )
}

// ------ ------
//     Start 
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    start_app("main", root);
}
