use std::collections::HashMap;
use std::sync::Arc;

use azalea::{BlockPos, Vec3};
use parking_lot::Mutex;
use serde_json::{Value, json};
use tokio::sync::broadcast;

use crate::config::Config;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Idle,
    Follow,
    Protect,
    Hold,
    Mine,
    Goto,
}

impl Mode {
    /// Lowercase wire name used in telemetry.
    pub fn as_str(self) -> &'static str {
        match self {
            Mode::Idle => "idle",
            Mode::Follow => "follow",
            Mode::Protect => "protect",
            Mode::Hold => "hold",
            Mode::Mine => "mine",
            Mode::Goto => "goto",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Stasis {
    pub id: u32,
    pub pos: BlockPos,
    pub owner: Option<String>,
}

/// Per-bot mutable state. Every account gets its own `Runtime`, so two bots can
/// never read or write each other's mode, targets, or task flags.
pub struct Runtime {
    pub mode: Mode,
    pub follow_target: Option<String>,
    pub protect_target: Option<String>,

    pub killaura_on: bool,
    pub killaura_range: f64,
    pub killaura_stay_still: bool,

    pub sel1: Option<BlockPos>,
    pub sel2: Option<BlockPos>,
    pub mining_active: bool,

    pub pull_request: Option<String>,
    pub pull_active: bool,
    pub stasis: Vec<Stasis>,
    pub next_stasis_id: u32,

    pub eat_cooldown: u32,
    pub telemetry_cooldown: u32,
    pub anti_afk: u32,

    /// In-game name of the player who last sent a whisper command, so async jobs
    /// can whisper their result/errors back to them. Cleared for GUI commands.
    pub command_sender: Option<String>,

    // --- advanced control features ---
    /// Keep a Totem of Undying in the offhand when possible.
    pub auto_totem: bool,
    /// Equip the best available armor pieces.
    pub auto_armor: bool,
    /// Disconnect when health drops to/below `auto_disconnect_health`.
    pub auto_disconnect: bool,
    pub auto_disconnect_health: f64,
    /// Active goto target (set when mode == Goto).
    pub goto_target: Option<BlockPos>,
    /// Last position we asked the pathfinder to walk to. Used to avoid
    /// restarting the path every tick (which floods the pathfinder and freezes
    /// the bot); we only re-path when the target actually moves.
    pub last_goal: Option<Vec3>,
    /// Runtime follow distance (radius), clamped to a sane range.
    pub follow_distance: f64,
    /// Whether the bot should be sprinting while moving.
    pub sprinting: bool,
    /// Whether the bot should be sneaking.
    pub sneaking: bool,
    /// One-shot request to deposit items into the nearest chest.
    pub deposit_request: bool,
    pub deposit_active: bool,
    /// One-shot request to drop trash items.
    pub drop_trash_request: bool,
    pub drop_trash_active: bool,
    /// Auto-eat trigger: the bot eats when its food level is at or below this
    /// value (real hunger, read from the game — not a blind timer).
    pub eat_threshold: u32,
    /// Throttle for auto_totem/auto_armor maintenance.
    pub equip_cooldown: u32,
    /// Throttle (in ticks) for the "I can't see you" warning, so a follow/
    /// protect command for a player who is not in this world (e.g. you are on a
    /// different server) reports once every few seconds instead of every tick.
    pub cant_see_cooldown: u32,
}

pub const DEFAULT_FOLLOW_DISTANCE: f64 = 2.0;
pub const DEFAULT_EAT_THRESHOLD: u32 = 17;
pub const MIN_FOLLOW_DISTANCE: f64 = 1.0;
pub const MAX_FOLLOW_DISTANCE: f64 = 10.0;
pub const MIN_EAT_THRESHOLD: u32 = 1;
pub const MAX_EAT_THRESHOLD: u32 = 19;

impl Default for Runtime {
    fn default() -> Self {
        Self {
            mode: Mode::Idle,
            follow_target: None,
            protect_target: None,
            killaura_on: false,
            killaura_range: 3.0,
            killaura_stay_still: true,
            sel1: None,
            sel2: None,
            mining_active: false,
            pull_request: None,
            pull_active: false,
            stasis: Vec::new(),
            next_stasis_id: 1,
            eat_cooldown: 0,
            telemetry_cooldown: 0,
            anti_afk: 0,
            command_sender: None,
            auto_totem: false,
            auto_armor: false,
            auto_disconnect: false,
            auto_disconnect_health: 0.0,
            goto_target: None,
            last_goal: None,
            follow_distance: DEFAULT_FOLLOW_DISTANCE,
            sprinting: false,
            sneaking: false,
            deposit_request: false,
            deposit_active: false,
            drop_trash_request: false,
            drop_trash_active: false,
            eat_threshold: DEFAULT_EAT_THRESHOLD,
            equip_cooldown: 0,
            cant_see_cooldown: 0,
        }
    }
}

/// Everything one bot owns: its account id (username), its isolated `Runtime`,
/// and a handle to the shared event channel so it can emit events tagged with
/// its own id. There is exactly one `BotCtx` per connected account.
pub struct BotCtx {
    pub id: String,
    pub rt: Mutex<Runtime>,
    tx: broadcast::Sender<String>,
}

impl BotCtx {
    fn new(id: String, tx: broadcast::Sender<String>) -> Self {
        Self {
            id,
            rt: Mutex::new(Runtime::default()),
            tx,
        }
    }

    pub fn emit(&self, json: String) {
        let _ = self.tx.send(json);
    }

    /// Chat / decision line for this specific bot.
    pub fn log(&self, text: &str) {
        self.emit(json!({ "event": "log", "bot": self.id, "text": text }).to_string());
    }

    /// A problem the operator should see (e.g. "I can't see you"). The mod
    /// surfaces these in your chat when the Notifications module is on.
    pub fn error(&self, text: &str) {
        self.emit(json!({ "event": "error", "bot": self.id, "text": text }).to_string());
    }

    /// Per-bot status (online / disconnected / ...).
    pub fn status(&self, status: &str) {
        self.emit(json!({ "event": "status", "bot": self.id, "status": status }).to_string());
    }

    pub fn telemetry(&self, health: f32, food: u32, x: f64, y: f64, z: f64) {
        let mode = self.rt.lock().mode.as_str();
        self.emit(
            json!({
                "event": "telemetry",
                "bot": self.id,
                "health": health,
                "food": food,
                "x": x,
                "y": y,
                "z": z,
                "mode": mode
            })
            .to_string(),
        );
    }

    /// Apply one already-parsed command to this bot's runtime.
    pub fn apply(&self, value: &Value) {
        let cmd = value.get("cmd").and_then(Value::as_str).unwrap_or("");
        let mut rt = self.rt.lock();
        // Commands from the mod GUI have no in-game sender to whisper back to.
        rt.command_sender = None;
        match cmd {
            "follow" => {
                rt.mode = Mode::Follow;
                rt.follow_target = str_field(value, "target");
            }
            "protect" => {
                rt.mode = Mode::Protect;
                rt.protect_target = str_field(value, "target");
            }
            "stop" => {
                rt.mode = Mode::Idle;
                rt.killaura_on = false;
                rt.mining_active = false;
                rt.goto_target = None;
                rt.last_goal = None;
            }
            "killaura" => {
                rt.killaura_on = bool_field(value, "on");
                if let Some(range) = value.get("range").and_then(Value::as_f64) {
                    rt.killaura_range = range.clamp(2.0, 6.0);
                }
                rt.killaura_stay_still = bool_field(value, "stay_still");
            }
            "hold_position" => {
                rt.mode = if bool_field(value, "on") {
                    Mode::Hold
                } else {
                    Mode::Idle
                };
            }
            "mine_start" => {
                rt.mode = Mode::Mine;
            }
            "pull" => {
                rt.pull_request = Some(str_field(value, "arg").unwrap_or_else(|| "manual".into()));
            }
            "auto_totem" => {
                rt.auto_totem = bool_field(value, "on");
            }
            "auto_armor" => {
                rt.auto_armor = bool_field(value, "on");
            }
            "auto_disconnect" => {
                rt.auto_disconnect = bool_field(value, "on");
                if let Some(health) = value.get("health").and_then(Value::as_f64) {
                    rt.auto_disconnect_health = health.max(0.0);
                }
            }
            "goto" => {
                let x = value.get("x").and_then(Value::as_f64);
                let y = value.get("y").and_then(Value::as_f64);
                let z = value.get("z").and_then(Value::as_f64);
                if let (Some(x), Some(y), Some(z)) = (x, y, z) {
                    rt.goto_target = Some(BlockPos::new(
                        x.floor() as i32,
                        y.floor() as i32,
                        z.floor() as i32,
                    ));
                    rt.mode = Mode::Goto;
                }
            }
            "follow_distance" => {
                if let Some(distance) = value.get("distance").and_then(Value::as_f64) {
                    rt.follow_distance = distance.clamp(MIN_FOLLOW_DISTANCE, MAX_FOLLOW_DISTANCE);
                }
            }
            "sprint" => {
                rt.sprinting = bool_field(value, "on");
            }
            "sneak" => {
                rt.sneaking = bool_field(value, "on");
            }
            "deposit" => {
                rt.deposit_request = true;
            }
            "drop_trash" => {
                rt.drop_trash_request = true;
            }
            "eat_threshold" => {
                if let Some(food) = value.get("food").and_then(Value::as_u64) {
                    rt.eat_threshold = (food as u32).clamp(MIN_EAT_THRESHOLD, MAX_EAT_THRESHOLD);
                }
            }
            _ => {}
        }
    }
}

/// Process-wide state shared by the swarm and the bridge. Holds the immutable
/// config, the single broadcast channel feeding the WebSocket, and the registry
/// of per-bot contexts keyed by account username.
pub struct Shared {
    pub config: Config,
    pub tx: broadcast::Sender<String>,
    bots: Mutex<HashMap<String, Arc<BotCtx>>>,
    /// In-game name allowed to command the bots by whisper. The mod sets this
    /// automatically to your own Minecraft name (see the `set_owner` command),
    /// so you never type it. Starts from the optional `owner` config field;
    /// empty means anyone who whispers may command the bot.
    owner: Mutex<String>,
}

impl Shared {
    pub fn new(config: Config) -> Self {
        let (tx, _rx) = broadcast::channel(256);
        let owner = Mutex::new(config.owner.clone());
        Self {
            config,
            tx,
            bots: Mutex::new(HashMap::new()),
            owner,
        }
    }

    /// The in-game name allowed to command the bots, or empty for "anyone".
    pub fn owner(&self) -> String {
        self.owner.lock().clone()
    }

    /// Set the owner (called when the mod reports your Minecraft name).
    pub fn set_owner(&self, name: &str) {
        *self.owner.lock() = name.to_string();
    }

    /// Get the context for `id`, creating it on first use. Each account resolves
    /// to a stable `BotCtx` for the lifetime of the process.
    pub fn ctx(&self, id: &str) -> Arc<BotCtx> {
        let mut bots = self.bots.lock();
        bots.entry(id.to_string())
            .or_insert_with(|| Arc::new(BotCtx::new(id.to_string(), self.tx.clone())))
            .clone()
    }

    /// Ids of every registered bot, used to announce the roster to the mod.
    pub fn ids(&self) -> Vec<String> {
        let mut ids: Vec<String> = self.bots.lock().keys().cloned().collect();
        ids.sort();
        ids
    }

    /// Process-level log line, not tied to any single bot (e.g. bridge events).
    pub fn log(&self, text: &str) {
        let _ = self
            .tx
            .send(json!({ "event": "log", "text": text }).to_string());
    }

    /// Bridge-link status (the WebSocket connection itself), not a bot's status.
    pub fn status(&self, status: &str) {
        let _ = self
            .tx
            .send(json!({ "event": "status", "status": status }).to_string());
    }

    /// Send the current roster of bot ids to the mod so the GUI can populate its
    /// account selector.
    pub fn announce_roster(&self) {
        let ids = self.ids();
        let _ = self
            .tx
            .send(json!({ "event": "roster", "bots": ids }).to_string());
    }

    /// Route a command coming from the bridge to its target bot(s).
    ///
    /// The optional `"bot"` field selects the target: a concrete id targets one
    /// account, `"all"`/`"*"` or an absent field broadcasts to every bot. An id
    /// that is not currently connected is ignored, so a stale GUI selection can
    /// never leak onto the wrong account.
    pub fn apply_command(&self, raw: &str) {
        let value: Value = match serde_json::from_str(raw) {
            Ok(value) => value,
            Err(_) => return,
        };
        // `set_owner` is global (it identifies the human player), so it is
        // handled here instead of being routed to a single bot's runtime.
        if value.get("cmd").and_then(Value::as_str) == Some("set_owner") {
            if let Some(name) = value.get("owner").and_then(Value::as_str) {
                self.set_owner(name);
            }
            return;
        }
        match value.get("bot").and_then(Value::as_str) {
            Some("all") | Some("*") => {
                for ctx in self.bots.lock().values() {
                    ctx.apply(&value);
                }
            }
            Some(id) => {
                let ctx = self.bots.lock().get(id).cloned();
                if let Some(ctx) = ctx {
                    ctx.apply(&value);
                }
            }
            None => {
                for ctx in self.bots.lock().values() {
                    ctx.apply(&value);
                }
            }
        }
    }
}

fn str_field(value: &Value, key: &str) -> Option<String> {
    value
        .get(key)
        .and_then(Value::as_str)
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
}

fn bool_field(value: &Value, key: &str) -> bool {
    value.get(key).and_then(Value::as_bool).unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;

    fn test_shared() -> Shared {
        Shared::new(Config {
            server: String::new(),
            data_dir: ".afk".into(),
            language: "en".into(),
            owner: String::new(),
            bridge_port: 0,
            reconnect_seconds: 8,
            accounts: Vec::new(),
        })
    }

    // Convenience: register a bot and read back its runtime mode.
    fn mode_of(s: &Shared, id: &str) -> Mode {
        s.ctx(id).rt.lock().mode
    }

    #[test]
    fn follow_sets_mode_and_target() {
        let s = test_shared();
        s.ctx("A");
        s.apply_command(r#"{"cmd":"follow","bot":"A","target":"Steve"}"#);
        let ctx = s.ctx("A");
        let rt = ctx.rt.lock();
        assert_eq!(rt.mode, Mode::Follow);
        assert_eq!(rt.follow_target.as_deref(), Some("Steve"));
    }

    #[test]
    fn protect_sets_mode_and_target() {
        let s = test_shared();
        s.ctx("A");
        s.apply_command(r#"{"cmd":"protect","bot":"A","target":"Alex"}"#);
        let ctx = s.ctx("A");
        let rt = ctx.rt.lock();
        assert_eq!(rt.mode, Mode::Protect);
        assert_eq!(rt.protect_target.as_deref(), Some("Alex"));
    }

    #[test]
    fn stop_resets_state() {
        let s = test_shared();
        s.ctx("A");
        s.apply_command(r#"{"cmd":"goto","bot":"A","x":1.0,"y":2.0,"z":3.0}"#);
        s.apply_command(r#"{"cmd":"killaura","bot":"A","on":true,"range":4.0,"stay_still":true}"#);
        s.apply_command(r#"{"cmd":"stop","bot":"A"}"#);
        let ctx = s.ctx("A");
        let rt = ctx.rt.lock();
        assert_eq!(rt.mode, Mode::Idle);
        assert!(!rt.killaura_on);
        assert!(!rt.mining_active);
        assert!(rt.goto_target.is_none());
    }

    #[test]
    fn killaura_parses_fields_and_clamps() {
        let s = test_shared();
        s.ctx("A");
        s.apply_command(r#"{"cmd":"killaura","bot":"A","on":true,"range":99.0,"stay_still":true}"#);
        let ctx = s.ctx("A");
        let rt = ctx.rt.lock();
        assert!(rt.killaura_on);
        assert_eq!(rt.killaura_range, 6.0); // clamped to max
        assert!(rt.killaura_stay_still);
    }

    #[test]
    fn hold_position_toggles_mode() {
        let s = test_shared();
        s.ctx("A");
        s.apply_command(r#"{"cmd":"hold_position","bot":"A","on":true}"#);
        assert_eq!(mode_of(&s, "A"), Mode::Hold);
        s.apply_command(r#"{"cmd":"hold_position","bot":"A","on":false}"#);
        assert_eq!(mode_of(&s, "A"), Mode::Idle);
    }

    #[test]
    fn mine_start_sets_mode() {
        let s = test_shared();
        s.ctx("A");
        s.apply_command(r#"{"cmd":"mine_start","bot":"A"}"#);
        assert_eq!(mode_of(&s, "A"), Mode::Mine);
    }

    #[test]
    fn pull_sets_request() {
        let s = test_shared();
        s.ctx("A");
        s.apply_command(r#"{"cmd":"pull","bot":"A","arg":"42"}"#);
        assert_eq!(s.ctx("A").rt.lock().pull_request.as_deref(), Some("42"));
        let s2 = test_shared();
        s2.ctx("A");
        s2.apply_command(r#"{"cmd":"pull","bot":"A"}"#);
        assert_eq!(s2.ctx("A").rt.lock().pull_request.as_deref(), Some("manual"));
    }

    #[test]
    fn auto_totem_toggle() {
        let s = test_shared();
        s.ctx("A");
        s.apply_command(r#"{"cmd":"auto_totem","bot":"A","on":true}"#);
        assert!(s.ctx("A").rt.lock().auto_totem);
        s.apply_command(r#"{"cmd":"auto_totem","bot":"A","on":false}"#);
        assert!(!s.ctx("A").rt.lock().auto_totem);
    }

    #[test]
    fn auto_armor_toggle() {
        let s = test_shared();
        s.ctx("A");
        s.apply_command(r#"{"cmd":"auto_armor","bot":"A","on":true}"#);
        assert!(s.ctx("A").rt.lock().auto_armor);
    }

    #[test]
    fn auto_disconnect_sets_threshold() {
        let s = test_shared();
        s.ctx("A");
        s.apply_command(r#"{"cmd":"auto_disconnect","bot":"A","on":true,"health":6.5}"#);
        let ctx = s.ctx("A");
        let rt = ctx.rt.lock();
        assert!(rt.auto_disconnect);
        assert_eq!(rt.auto_disconnect_health, 6.5);
    }

    #[test]
    fn goto_floors_coords_and_sets_mode() {
        let s = test_shared();
        s.ctx("A");
        s.apply_command(r#"{"cmd":"goto","bot":"A","x":10.9,"y":64.2,"z":-3.7}"#);
        let ctx = s.ctx("A");
        let rt = ctx.rt.lock();
        assert_eq!(rt.mode, Mode::Goto);
        assert_eq!(rt.goto_target, Some(BlockPos::new(10, 64, -4)));
    }

    #[test]
    fn follow_distance_clamps() {
        let s = test_shared();
        s.ctx("A");
        s.apply_command(r#"{"cmd":"follow_distance","bot":"A","distance":0.1}"#);
        assert_eq!(s.ctx("A").rt.lock().follow_distance, MIN_FOLLOW_DISTANCE);
        s.apply_command(r#"{"cmd":"follow_distance","bot":"A","distance":50.0}"#);
        assert_eq!(s.ctx("A").rt.lock().follow_distance, MAX_FOLLOW_DISTANCE);
        s.apply_command(r#"{"cmd":"follow_distance","bot":"A","distance":4.0}"#);
        assert_eq!(s.ctx("A").rt.lock().follow_distance, 4.0);
    }

    #[test]
    fn sprint_toggle() {
        let s = test_shared();
        s.ctx("A");
        s.apply_command(r#"{"cmd":"sprint","bot":"A","on":true}"#);
        assert!(s.ctx("A").rt.lock().sprinting);
        s.apply_command(r#"{"cmd":"sprint","bot":"A","on":false}"#);
        assert!(!s.ctx("A").rt.lock().sprinting);
    }

    #[test]
    fn sneak_toggle() {
        let s = test_shared();
        s.ctx("A");
        s.apply_command(r#"{"cmd":"sneak","bot":"A","on":true}"#);
        assert!(s.ctx("A").rt.lock().sneaking);
    }

    #[test]
    fn deposit_sets_request() {
        let s = test_shared();
        s.ctx("A");
        s.apply_command(r#"{"cmd":"deposit","bot":"A"}"#);
        assert!(s.ctx("A").rt.lock().deposit_request);
    }

    #[test]
    fn drop_trash_sets_request() {
        let s = test_shared();
        s.ctx("A");
        s.apply_command(r#"{"cmd":"drop_trash","bot":"A"}"#);
        assert!(s.ctx("A").rt.lock().drop_trash_request);
    }

    #[test]
    fn eat_threshold_sets_and_clamps() {
        let s = test_shared();
        s.ctx("A");
        s.apply_command(r#"{"cmd":"eat_threshold","bot":"A","food":15}"#);
        assert_eq!(s.ctx("A").rt.lock().eat_threshold, 15);
        s.apply_command(r#"{"cmd":"eat_threshold","bot":"A","food":0}"#);
        assert_eq!(s.ctx("A").rt.lock().eat_threshold, MIN_EAT_THRESHOLD);
        s.apply_command(r#"{"cmd":"eat_threshold","bot":"A","food":99}"#);
        assert_eq!(s.ctx("A").rt.lock().eat_threshold, MAX_EAT_THRESHOLD);
    }

    #[test]
    fn unknown_command_is_ignored() {
        let s = test_shared();
        s.ctx("A");
        s.apply_command(r#"{"cmd":"nonsense","bot":"A"}"#);
        assert_eq!(mode_of(&s, "A"), Mode::Idle);
    }

    #[test]
    fn malformed_json_is_ignored() {
        let s = test_shared();
        s.ctx("A");
        s.apply_command("not json at all");
        assert_eq!(mode_of(&s, "A"), Mode::Idle);
    }

    #[test]
    fn commands_are_isolated_per_bot() {
        let s = test_shared();
        s.ctx("A");
        s.ctx("B");
        // A command aimed at A must never touch B.
        s.apply_command(r#"{"cmd":"follow","bot":"A","target":"Steve"}"#);
        assert_eq!(mode_of(&s, "A"), Mode::Follow);
        assert_eq!(mode_of(&s, "B"), Mode::Idle);
        assert!(s.ctx("B").rt.lock().follow_target.is_none());

        // And a command aimed at B must never touch A.
        s.apply_command(r#"{"cmd":"hold_position","bot":"B","on":true}"#);
        assert_eq!(mode_of(&s, "B"), Mode::Hold);
        assert_eq!(mode_of(&s, "A"), Mode::Follow);
    }

    #[test]
    fn broadcast_targets_every_bot() {
        let s = test_shared();
        s.ctx("A");
        s.ctx("B");
        s.apply_command(r#"{"cmd":"hold_position","bot":"all","on":true}"#);
        assert_eq!(mode_of(&s, "A"), Mode::Hold);
        assert_eq!(mode_of(&s, "B"), Mode::Hold);
    }

    #[test]
    fn command_for_unknown_bot_is_ignored() {
        let s = test_shared();
        s.ctx("A");
        // Target an account that is not connected: nothing must change, and no
        // ghost context is created for it.
        s.apply_command(r#"{"cmd":"follow","bot":"ghost","target":"Steve"}"#);
        assert_eq!(mode_of(&s, "A"), Mode::Idle);
        assert_eq!(s.ids(), vec!["A".to_string()]);
    }

    #[test]
    fn set_owner_is_global_and_creates_no_bot() {
        let s = test_shared();
        assert_eq!(s.owner(), "");
        s.apply_command(r#"{"cmd":"set_owner","owner":"Steve","bot":"all"}"#);
        assert_eq!(s.owner(), "Steve");
        // It must not have created a phantom bot for the "all" tag.
        assert!(s.ids().is_empty());
    }

    #[test]
    fn mode_as_str_mapping() {
        assert_eq!(Mode::Idle.as_str(), "idle");
        assert_eq!(Mode::Follow.as_str(), "follow");
        assert_eq!(Mode::Protect.as_str(), "protect");
        assert_eq!(Mode::Hold.as_str(), "hold");
        assert_eq!(Mode::Mine.as_str(), "mine");
        assert_eq!(Mode::Goto.as_str(), "goto");
    }
}
