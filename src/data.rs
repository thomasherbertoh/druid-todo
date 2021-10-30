use druid::{Data, Env, EventCtx, Lens};
use serde::{Deserialize, Serialize};
use serde_json::Error;
use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;
use uuid::Uuid;

use crate::delegate::DELETE;

#[derive(Clone, Data, Lens)]
pub struct AppState {
	new_todo: String,
	todos: Arc<Vec<TodoItem>>,
}

impl AppState {
	pub fn _new(todos: Arc<Vec<TodoItem>>) -> Self {
		Self {
			new_todo: "".into(),
			todos: Arc::from(todos),
		}
	}

	fn add_todo(&mut self) {
		Arc::make_mut(&mut self.todos).insert(0, TodoItem::new(&self.new_todo));
		self.new_todo = "".into();
		self.save_to_json().unwrap();
	}

	pub fn delete_todo(&mut self, id: &Uuid) {
		Arc::make_mut(&mut self.todos).retain(|item| &item.id != id);
		self.save_to_json().unwrap();
	}

	pub fn click_add(_ctx: &mut EventCtx, data: &mut Self, _env: &Env) {
		if data.new_todo != "" {
			data.add_todo();
		}
	}

	pub fn save_to_json(&mut self) -> Result<(), Error> {
		let serialised = serde_json::to_string_pretty(Arc::make_mut(&mut self.todos))?;
		std::fs::write("todos.json", serialised).unwrap();
		Ok(())
	}

	pub fn load_from_json() -> Self {
		let file = File::open("todos.json");

		match file {
			Ok(file) => {
				let reader = BufReader::new(file);
				let todos: Vec<TodoItem> = serde_json::from_reader(reader).unwrap_or(vec![]);
				Self {
					todos: Arc::from(todos),
					new_todo: String::new(),
				}
			}
			Err(_) => Self {
				todos: Arc::new(Vec::new()),
				new_todo: String::new(),
			},
		}
	}

	pub fn clear_completed(_ctx: &mut EventCtx, data: &mut Self, _env: &Env) {
		Arc::make_mut(&mut data.todos).retain(|item| !item.done);
		data.save_to_json().unwrap();
	}
}

#[derive(Clone, Data, Lens, Serialize, Deserialize)]
pub struct TodoItem {
	#[data(same_fn = "PartialEq::eq")]
	pub id: Uuid,
	pub done: bool,
	text: String,
}

impl TodoItem {
	pub fn new(text: &str) -> Self {
		Self {
			id: Uuid::new_v4(),
			done: false,
			text: text.into(),
		}
	}

	pub fn click_delete(ctx: &mut EventCtx, data: &mut Self, _env: &Env) {
		ctx.submit_command(DELETE.with(data.id));
	}
}
