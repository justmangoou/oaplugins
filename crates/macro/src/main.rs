use enigo::{Button as MouseButton, Direction, Enigo, Mouse, Settings};
use openaction::{
	Action, ActionUuid, Instance, InstanceId, OpenActionResult, async_trait, register_action,
};
use serde::{Deserialize, Serialize};
use std::{sync::LazyLock, time::Duration};
use tokio::{
	sync::{Mutex, RwLock},
	time::sleep,
};

static ENIGO: LazyLock<Mutex<Enigo>> = LazyLock::new(|| {
	let settings = Settings {
		linux_delay: 0,
		..Default::default()
	};

	Mutex::new(Enigo::new(&settings).unwrap())
});
static RUNNING_INSTANCE: LazyLock<RwLock<Option<InstanceId>>> = LazyLock::new(|| RwLock::new(None));

#[tokio::main]
async fn main() -> OpenActionResult<()> {
	register_action(AutoMouseClickAction).await;
	openaction::run(std::env::args().collect()).await
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub enum ClickType {
	#[default]
	Single,
	Double,
	Hold,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct AutoMouseClickSettings {
	pub click_type: ClickType,
	pub button: MouseButton,
	pub interval: u64,
}

impl Default for AutoMouseClickSettings {
	fn default() -> Self {
		Self {
			click_type: ClickType::default(),
			button: MouseButton::default(),
			interval: 100,
		}
	}
}

async fn perform_click(button: MouseButton, click_type: &ClickType) {
	let mut enigo = ENIGO.lock().await;

	match click_type {
		ClickType::Single => {
			let _ = enigo.button(button, Direction::Click);
		}
		ClickType::Double => {
			let _ = enigo.button(button, Direction::Click);
			sleep(Duration::from_millis(50)).await;
			let _ = enigo.button(button, Direction::Click);
		}
		ClickType::Hold => unimplemented!(),
	}
}

pub struct AutoMouseClickAction;

#[async_trait]
impl Action for AutoMouseClickAction {
	const UUID: ActionUuid = "com.github.justmangoou.oamacro.mouse";
	type Settings = AutoMouseClickSettings;

	async fn key_up(&self, instance: &Instance, settings: &Self::Settings) -> OpenActionResult<()> {
		let id = instance.instance_id.clone();

		let should_start = {
			let mut running = RUNNING_INSTANCE.write().await;
			match *running {
				Some(_) => {
					*running = None;
					false
				}
				None => {
					*running = Some(id.clone());
					true
				}
			}
		};

		if should_start {
			let interval = settings.interval;
			let button = settings.button;
			let click_type = settings.click_type.clone();

			tokio::spawn(async move {
				loop {
					let current = RUNNING_INSTANCE.read().await;

					if current.as_ref() != Some(&id) {
						break;
					}
					drop(current);

					perform_click(button, &click_type).await;
					sleep(Duration::from_millis(interval)).await;
				}
			});
		}

		Ok(())
	}
}
