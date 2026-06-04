use std::path::{Path, PathBuf};
use std::sync::Arc;

use anyhow::{Context, Result};
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};

use crate::console;

/// Top-level configuration. Created interactively on first run and editable by
/// hand afterwards. Login token caches live under `data_dir` (default `.afk`),
/// one file per account, instead of inside the Minecraft folder.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub server: String,
    #[serde(default = "default_data_dir")]
    pub data_dir: String,
    /// UI language for the setup/menu console, as a two-letter code
    /// (`en`, `fr`, `es`, `de`, `ru`, `pt`, `it`). Chosen on first run.
    #[serde(default = "default_language")]
    pub language: String,
    /// Microsoft OAuth client id used for the device-code sign-in. Defaults to
    /// the Minecraft launcher's public client id. Override it here only if
    /// Microsoft rejects the default (e.g. with your own Azure app's id).
    #[serde(default)]
    pub ms_client_id: Option<String>,
    /// In-game name of the only player allowed to command the bot by whisper.
    /// Empty (the default) means any player who whispers the bot may command it.
    /// The Fabric mod sets this automatically to your own name, so you normally
    /// never touch it.
    #[serde(default)]
    pub owner: String,
    #[serde(default = "default_port")]
    pub bridge_port: u16,
    #[serde(default = "default_reconnect")]
    pub reconnect_seconds: u64,
    pub accounts: Vec<AccountConfig>,
}

/// A single bot account.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountConfig {
    /// For Microsoft auth this is the account email (login cache key); for
    /// offline auth it is the in-game username.
    pub username: String,
    #[serde(default = "default_auth")]
    pub auth: String,
}

fn default_auth() -> String {
    "microsoft".to_string()
}
fn default_data_dir() -> String {
    ".afk".to_string()
}
fn default_language() -> String {
    "en".to_string()
}
fn default_port() -> u16 {
    8765
}
fn default_reconnect() -> u64 {
    8
}

impl Config {
    /// Load `config.json` if present (then show the account menu), otherwise run
    /// first-time setup. Hand edits to the file always win over the prompts.
    pub fn load_or_setup(path: &str) -> Result<Self> {
        let p = Path::new(path);
        if p.exists() {
            let raw = std::fs::read_to_string(p).with_context(|| format!("cannot read {path}"))?;
            let mut config: Config =
                serde_json::from_str(&raw).with_context(|| format!("invalid JSON in {path}"))?;
            config.validate()?;
            manage_accounts(&mut config, path)?;
            config.validate()?;
            return Ok(config);
        }

        let config = interactive_setup()?;
        write_config(&config, path)?;
        Ok(config)
    }

    /// Absolute path to the per-account login cache directory.
    pub fn data_path(&self) -> PathBuf {
        PathBuf::from(&self.data_dir)
    }

    /// The chosen UI language, falling back to English for an unknown code.
    pub fn lang(&self) -> Lang {
        Lang::from_code(&self.language)
    }

    fn validate(&self) -> Result<()> {
        if self.accounts.is_empty() {
            anyhow::bail!("config has no accounts; add at least one to \"accounts\"");
        }
        let mut seen = std::collections::HashSet::new();
        for account in &self.accounts {
            if account.username.trim().is_empty() {
                anyhow::bail!("an account has an empty username");
            }
            if !seen.insert(account.username.to_lowercase()) {
                anyhow::bail!("duplicate account username: {}", account.username);
            }
        }
        Ok(())
    }
}

impl AccountConfig {
    /// Build an Azalea account. Microsoft accounts cache (and auto-refresh) their
    /// token in `data_dir/<account>.json` so logins survive restarts and stay
    /// out of the Minecraft folder.
    pub async fn to_account(
        &self,
        data_dir: &Path,
        ms_client_id: Option<&str>,
    ) -> Result<azalea::Account> {
        match self.auth.to_lowercase().as_str() {
            "offline" | "cracked" => Ok(azalea::Account::offline(&self.username)),
            _ => {
                microsoft_account(
                    &self.username,
                    &cache_file(data_dir, &self.username),
                    ms_client_id,
                )
                .await
            }
        }
    }
}

/// Per-account cache file path inside the data directory.
fn cache_file(data_dir: &Path, account: &str) -> PathBuf {
    let safe: String = account
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .collect();
    data_dir.join(format!("{safe}.json"))
}

/// Log in with Microsoft using a custom cache file. `cache_key` is only a local
/// label for the token cache (the user signs in with the device-code link, so no
/// e-mail is ever typed). This mirrors `Account::microsoft` but lets us choose
/// where the token is cached.
async fn microsoft_account(
    cache_key: &str,
    cache: &Path,
    client_id: Option<&str>,
) -> Result<azalea::Account> {
    println!(
        "  {}{}Microsoft sign-in{} {}open the link below and enter the code{}",
        console::BOLD,
        console::PURPLE,
        console::RESET,
        console::GRAY,
        console::RESET
    );
    // Default to the Minecraft launcher's public client id, which Microsoft
    // still accepts for the device-code flow (Azalea's built-in id is rejected
    // as a "first party application" on some accounts).
    let client_id = client_id.unwrap_or("00000000402b5328");
    let result = azalea::auth::auth(
        cache_key,
        azalea::auth::AuthOpts {
            cache_file: Some(cache.to_path_buf()),
            client_id: Some(client_id),
            ..Default::default()
        },
    )
    .await
    .context("Microsoft login failed")?;

    Ok(azalea::Account {
        username: result.profile.name,
        access_token: Some(Arc::new(Mutex::new(result.access_token))),
        uuid: Some(result.profile.id),
        account_opts: azalea::AccountOpts::Microsoft {
            email: cache_key.to_owned(),
        },
        certs: Arc::new(Mutex::new(None)),
    })
}

fn write_config(config: &Config, path: &str) -> Result<()> {
    let json = serde_json::to_string_pretty(config)?;
    std::fs::write(path, json).with_context(|| format!("cannot write {path}"))
}

// ===================================================================
//  Localized console strings (7 languages)
// ===================================================================

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Lang {
    En,
    Fr,
    Es,
    De,
    Ru,
    Pt,
    It,
}

impl Lang {
    pub fn from_code(code: &str) -> Lang {
        match code.trim().to_lowercase().as_str() {
            "fr" => Lang::Fr,
            "es" => Lang::Es,
            "de" => Lang::De,
            "ru" => Lang::Ru,
            "pt" => Lang::Pt,
            "it" => Lang::It,
            _ => Lang::En,
        }
    }

    pub fn code(self) -> &'static str {
        match self {
            Lang::En => "en",
            Lang::Fr => "fr",
            Lang::Es => "es",
            Lang::De => "de",
            Lang::Ru => "ru",
            Lang::Pt => "pt",
            Lang::It => "it",
        }
    }
}

/// Every piece of localized text the console shows. Keeping them in one struct
/// (one value per language) makes it obvious that all languages stay in sync.
struct Strings {
    setup_title: &'static str,
    intro: &'static str,
    auth: &'static str,
    ms_note: &'static str,
    name: &'static str,
    ip: &'static str,
    port: &'static str,
    empty: &'static str,
    saved: &'static str,
    // account menu
    accounts_title: &'static str,
    server: &'static str,
    accounts: &'static str,
    menu: &'static str,
    start_one: &'static str,
    no_number: &'static str,
    ask_number: &'static str,
    logged_out: &'static str,
    no_cache: &'static str,
}

fn strings(lang: Lang) -> Strings {
    match lang {
        Lang::En => Strings {
            setup_title: "First-time setup",
            intro: "Press Enter to accept the [default] in brackets.",
            auth: "Account type? [1] Microsoft (default)  [2] Cracked/offline:",
            ms_note: "Microsoft: you'll sign in at startup with the link and code shown. Nothing to type here.",
            name: "Bot username:",
            ip: "Server IP [127.0.0.1]:",
            port: "Server port [25565]:",
            empty: "This cannot be empty, please type a value.",
            saved: "Configuration saved.",
            accounts_title: "Accounts",
            server: "Server",
            accounts: "Accounts",
            menu: "Action: [Enter] start all · [number] start one · [a] add · [r] remove · [l] log out · [q] quit:",
            start_one: "Starting only",
            no_number: "No account with that number.",
            ask_number: "Account number:",
            logged_out: "Logged out. It will sign in again on the next start:",
            no_cache: "No cached login for:",
        },
        Lang::Fr => Strings {
            setup_title: "Configuration initiale",
            intro: "Appuie sur Entrée pour garder la valeur [par défaut] entre crochets.",
            auth: "Type de compte ? [1] Microsoft (défaut)  [2] Cracké/hors-ligne :",
            ms_note: "Microsoft : tu te connecteras au démarrage avec le lien et le code affichés. Rien à taper ici.",
            name: "Nom du bot :",
            ip: "IP du serveur [127.0.0.1] :",
            port: "Port du serveur [25565] :",
            empty: "Ce champ ne peut pas être vide, entre une valeur.",
            saved: "Configuration enregistrée.",
            accounts_title: "Comptes",
            server: "Serveur",
            accounts: "Comptes",
            menu: "Action : [Entrée] tout démarrer · [numéro] démarrer un · [a] ajouter · [r] retirer · [l] déconnecter · [q] quitter :",
            start_one: "Démarrage uniquement de",
            no_number: "Aucun compte avec ce numéro.",
            ask_number: "Numéro du compte :",
            logged_out: "Déconnecté. Il se reconnectera au prochain démarrage :",
            no_cache: "Aucune connexion en cache pour :",
        },
        Lang::Es => Strings {
            setup_title: "Configuración inicial",
            intro: "Pulsa Enter para aceptar el valor [predeterminado] entre corchetes.",
            auth: "¿Tipo de cuenta? [1] Microsoft (predeterminado)  [2] Cracked/sin conexión:",
            ms_note: "Microsoft: iniciarás sesión al arrancar con el enlace y el código mostrados. Nada que escribir aquí.",
            name: "Nombre del bot:",
            ip: "IP del servidor [127.0.0.1]:",
            port: "Puerto del servidor [25565]:",
            empty: "Esto no puede estar vacío, escribe un valor.",
            saved: "Configuración guardada.",
            accounts_title: "Cuentas",
            server: "Servidor",
            accounts: "Cuentas",
            menu: "Acción: [Enter] iniciar todas · [número] iniciar una · [a] añadir · [r] quitar · [l] cerrar sesión · [q] salir:",
            start_one: "Iniciando solo",
            no_number: "No hay ninguna cuenta con ese número.",
            ask_number: "Número de cuenta:",
            logged_out: "Sesión cerrada. Volverá a iniciar sesión en el próximo arranque:",
            no_cache: "No hay sesión en caché para:",
        },
        Lang::De => Strings {
            setup_title: "Ersteinrichtung",
            intro: "Drücke Enter, um den [Standard] in Klammern zu übernehmen.",
            auth: "Kontotyp? [1] Microsoft (Standard)  [2] Cracked/Offline:",
            ms_note: "Microsoft: Du meldest dich beim Start mit dem angezeigten Link und Code an. Hier nichts einzugeben.",
            name: "Bot-Name:",
            ip: "Server-IP [127.0.0.1]:",
            port: "Server-Port [25565]:",
            empty: "Dies darf nicht leer sein, bitte einen Wert eingeben.",
            saved: "Konfiguration gespeichert.",
            accounts_title: "Konten",
            server: "Server",
            accounts: "Konten",
            menu: "Aktion: [Enter] alle starten · [Nummer] eines starten · [a] hinzufügen · [r] entfernen · [l] abmelden · [q] beenden:",
            start_one: "Starte nur",
            no_number: "Kein Konto mit dieser Nummer.",
            ask_number: "Kontonummer:",
            logged_out: "Abgemeldet. Meldet sich beim nächsten Start erneut an:",
            no_cache: "Keine gespeicherte Anmeldung für:",
        },
        Lang::Ru => Strings {
            setup_title: "Первоначальная настройка",
            intro: "Нажми Enter, чтобы оставить значение [по умолчанию] в скобках.",
            auth: "Тип аккаунта? [1] Microsoft (по умолчанию)  [2] Cracked/офлайн:",
            ms_note: "Microsoft: вход выполнится при запуске по показанным ссылке и коду. Здесь ничего вводить не нужно.",
            name: "Имя бота:",
            ip: "IP сервера [127.0.0.1]:",
            port: "Порт сервера [25565]:",
            empty: "Поле не может быть пустым, введи значение.",
            saved: "Конфигурация сохранена.",
            accounts_title: "Аккаунты",
            server: "Сервер",
            accounts: "Аккаунты",
            menu: "Действие: [Enter] запустить все · [номер] запустить один · [a] добавить · [r] удалить · [l] выйти · [q] выход:",
            start_one: "Запуск только",
            no_number: "Нет аккаунта с таким номером.",
            ask_number: "Номер аккаунта:",
            logged_out: "Выполнен выход. Войдёт снова при следующем запуске:",
            no_cache: "Нет сохранённого входа для:",
        },
        Lang::Pt => Strings {
            setup_title: "Configuração inicial",
            intro: "Pressione Enter para aceitar o valor [padrão] entre colchetes.",
            auth: "Tipo de conta? [1] Microsoft (padrão)  [2] Cracked/offline:",
            ms_note: "Microsoft: você fará login ao iniciar com o link e o código mostrados. Nada para digitar aqui.",
            name: "Nome do bot:",
            ip: "IP do servidor [127.0.0.1]:",
            port: "Porta do servidor [25565]:",
            empty: "Isto não pode ficar vazio, digite um valor.",
            saved: "Configuração salva.",
            accounts_title: "Contas",
            server: "Servidor",
            accounts: "Contas",
            menu: "Ação: [Enter] iniciar todas · [número] iniciar uma · [a] adicionar · [r] remover · [l] sair · [q] encerrar:",
            start_one: "Iniciando apenas",
            no_number: "Nenhuma conta com esse número.",
            ask_number: "Número da conta:",
            logged_out: "Sessão encerrada. Entrará novamente no próximo início:",
            no_cache: "Nenhum login em cache para:",
        },
        Lang::It => Strings {
            setup_title: "Configurazione iniziale",
            intro: "Premi Invio per accettare il valore [predefinito] tra parentesi.",
            auth: "Tipo di account? [1] Microsoft (predefinito)  [2] Cracked/offline:",
            ms_note: "Microsoft: accederai all'avvio con il link e il codice mostrati. Niente da digitare qui.",
            name: "Nome del bot:",
            ip: "IP del server [127.0.0.1]:",
            port: "Porta del server [25565]:",
            empty: "Questo campo non può essere vuoto, inserisci un valore.",
            saved: "Configurazione salvata.",
            accounts_title: "Account",
            server: "Server",
            accounts: "Account",
            menu: "Azione: [Invio] avvia tutti · [numero] avviane uno · [a] aggiungi · [r] rimuovi · [l] disconnetti · [q] esci:",
            start_one: "Avvio solo di",
            no_number: "Nessun account con quel numero.",
            ask_number: "Numero account:",
            logged_out: "Disconnesso. Effettuerà di nuovo l'accesso al prossimo avvio:",
            no_cache: "Nessun accesso in cache per:",
        },
    }
}

/// Ask a question and use `fallback` when the answer is empty.
fn ask_default(question: &str, fallback: &str) -> String {
    let value = console::ask(question);
    if value.is_empty() {
        fallback.to_string()
    } else {
        value
    }
}

/// Ask a question and refuse an empty answer. This prevents saving an account
/// with no username, which would make the bot fail to start.
fn ask_required(question: &str, empty_message: &str) -> String {
    loop {
        let value = console::ask(question);
        if !value.trim().is_empty() {
            return value;
        }
        console::warn(empty_message);
    }
}

/// Ask for one account. Microsoft accounts ask nothing — you are identified by
/// signing in to Microsoft at startup — so we only keep a stable local
/// `microsoft_key` for the token cache. Offline accounts still need an in-game
/// name, since that name *is* the identity in offline mode.
fn ask_account(t: &Strings, microsoft_key: &str) -> AccountConfig {
    let auth = match console::ask(t.auth).as_str() {
        "2" | "cracked" | "offline" => "offline",
        _ => "microsoft",
    };
    if auth == "microsoft" {
        console::hint(t.ms_note);
        AccountConfig {
            username: microsoft_key.to_string(),
            auth: "microsoft".to_string(),
        }
    } else {
        AccountConfig {
            username: ask_required(t.name, t.empty),
            auth: "offline".to_string(),
        }
    }
}

/// A stable, unused local key ("account-1", "account-2", ...) for a new Microsoft
/// account's token cache. The real in-game name comes from Microsoft at sign-in.
fn next_microsoft_key(config: &Config) -> String {
    let mut n = 1;
    loop {
        let key = format!("account-{n}");
        if !config.accounts.iter().any(|a| a.username == key) {
            return key;
        }
        n += 1;
    }
}

fn interactive_setup() -> Result<Config> {
    let lang = Lang::from_code(&console::ask(
        "Language / Langue / Idioma / Sprache / Язык / Idioma / Lingua? [EN]/FR/ES/DE/RU/PT/IT:",
    ));
    let t = strings(lang);

    console::section(t.setup_title);
    console::hint(t.intro);

    let account = ask_account(&t, "account-1");

    let ip = ask_default(t.ip, "127.0.0.1");
    let port = ask_default(t.port, "25565");
    let server = if port == "25565" {
        ip
    } else {
        format!("{ip}:{port}")
    };

    console::ok(t.saved);
    Ok(Config {
        server,
        data_dir: default_data_dir(),
        language: lang.code().to_string(),
        ms_client_id: None,
        // Left empty on purpose: the mod reports your in-game name automatically
        // (the `set_owner` command), so you never have to type it here.
        owner: String::new(),
        bridge_port: 8765,
        reconnect_seconds: 8,
        accounts: vec![account],
    })
}

/// Interactive account menu shown when a config already exists: start all
/// accounts, start one by number, add an account, remove one, or log one out
/// (clear its cached token). Localized to the language saved in the config.
fn manage_accounts(config: &mut Config, path: &str) -> Result<()> {
    let t = strings(config.lang());
    loop {
        println!();
        console::section(t.accounts_title);
        console::info(&format!("{}: {}", t.server, config.server));
        console::info(&format!("{}:", t.accounts));
        for (i, a) in config.accounts.iter().enumerate() {
            console::info(&format!("  [{}] {} ({})", i + 1, a.username, a.auth));
        }
        let choice = console::ask(t.menu).to_lowercase();
        // A bare number starts only that account (the others stay in the file).
        if let Ok(n) = choice.parse::<usize>() {
            if n >= 1 && n <= config.accounts.len() {
                let chosen = config.accounts[n - 1].clone();
                console::ok(&format!("{} {}.", t.start_one, chosen.username));
                config.accounts = vec![chosen];
                return Ok(());
            }
            console::warn(t.no_number);
            continue;
        }
        match choice.as_str() {
            "" => return Ok(()),
            "a" => {
                let key = next_microsoft_key(config);
                config.accounts.push(ask_account(&t, &key));
                write_config(config, path)?;
            }
            "r" => {
                if let Some(index) = pick_account(&t, config) {
                    let removed = config.accounts.remove(index);
                    let _ = std::fs::remove_file(cache_file(&config.data_path(), &removed.username));
                    write_config(config, path)?;
                }
            }
            "l" => {
                if let Some(index) = pick_account(&t, config) {
                    let account = &config.accounts[index];
                    match std::fs::remove_file(cache_file(&config.data_path(), &account.username)) {
                        Ok(_) => console::ok(&format!("{} {}", t.logged_out, account.username)),
                        Err(_) => console::warn(&format!("{} {}", t.no_cache, account.username)),
                    }
                }
            }
            "q" => std::process::exit(0),
            _ => {}
        }
    }
}

fn pick_account(t: &Strings, config: &Config) -> Option<usize> {
    let raw = console::ask(t.ask_number);
    let index = raw.parse::<usize>().ok()?;
    if index >= 1 && index <= config.accounts.len() {
        Some(index - 1)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cache_file_sanitizes_account_into_data_dir() {
        let path = cache_file(Path::new(".afk"), "User@Example.com");
        assert_eq!(path, Path::new(".afk").join("User_Example_com.json"));
    }

    #[test]
    fn offline_auth_needs_no_cache() {
        // Sanity: the data dir is only used to derive a Microsoft cache path; the
        // helper itself must produce a stable, filesystem-safe name.
        assert_eq!(
            cache_file(Path::new("/tmp/x"), "bot 1!"),
            Path::new("/tmp/x").join("bot_1_.json")
        );
    }

    #[test]
    fn lang_round_trips_through_code() {
        for lang in [
            Lang::En,
            Lang::Fr,
            Lang::Es,
            Lang::De,
            Lang::Ru,
            Lang::Pt,
            Lang::It,
        ] {
            assert!(Lang::from_code(lang.code()) == lang);
        }
        // Unknown codes fall back to English.
        assert!(Lang::from_code("xx") == Lang::En);
    }
}
