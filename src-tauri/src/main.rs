use std::process::Command;
use std::env::{self, set_var, var};
use std::fs::{self, File};
use std::io::{BufReader, BufWriter, Write};
use std::path::Path;
use serde::{Serialize, Deserialize};
use std::io;

mod data;

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    lang: String,
    theme: String,
    lightmode: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            lang: system_lang(),
            theme: "default".to_string(),
            lightmode: false,
        }
    }
}

//Открытие программ

#[tauri::command]
fn startprog(mode: &str) -> String {
    let mut cmd = match mode {
        "driver" => Command::new("enos-driver-manager"),
        "pamac" => Command::new("pamac-manager"),
        "rassist" => Command::new("enos-assistant-creator"),
        "zapret" => Command::new("enos-zapret-manager"),
        _ => return "Что то тут не так..".to_string()
    };

    match cmd.spawn() {
        Ok(..) => "OK".to_string().to_string(),
        Err(e) => format!("Ошибка {e}").to_string()
    }
    // ЭТО НЕ ИИШКА
}

#[tauri::command]
fn tweak(name: &str) -> Result<String, String> {
    match name {
        "kde_session" => Ok("sss".to_string()),
        "keys_on" => {
            run_cmd("pkexec", &["pacman-key-manager", "--install"])
            .map(|_| "Ключи установлены".to_string())
            .map_err(|e| e.to_string())
        }
        "keys_off" => {
            run_cmd("pkexec", &["pacman-key-manager", "--uninstall"])
            .map(|_| "Ключи удалены".to_string())
            .map_err(|e| e.to_string())
        }
        "zram_on" => {
            run_cmd("pkexec", &["zram-manager", "--install"])
            .map(|_| "ZRAM включен".to_string())
            .map_err(|e| e.to_string())
        }
        "zram_off" => {
            run_cmd("pkexec", &["zram-manager", "--uninstall"])
            .map(|_| "ZRAM выключен".to_string())
            .map_err(|e| e.to_string())
        }
        "refresh_mirrors" => {
            run_cmd("pkexec", &["reflector", "--latest", "20", "--protocol", "https", "--sort", "rate", "--save", "/etc/pacman.d/mirrorlist"])
            .map(|_| "Зеркала обновлены".to_string())
            .map_err(|e| e.to_string())
        }
        "clear_journal" => {
            run_cmd("pkexec", &["journalctl", "--vacuum-time=2weeks"])
            .map(|_| "Журнал очищен".to_string())
            .map_err(|e| e.to_string())
        }
        _ => Err("Неизвестная команда".to_string()),
    }
}

#[tauri::command]
fn check_prog(name: &str) -> bool {
    match name {
        "zram-manager" => {
            let path = "/etc/systemd/system/zram.service";
            let path = Path::new(&path);
            path.exists()
        }
        "pacman-key-manager" => {
            let path = "/etc/systemd/system/pacman-key-manager.service";
            let path = Path::new(&path);
            path.exists()
        }
        _ => false
    }
}

fn run_cmd(cmd: &str, args: &[&str]) -> io::Result<()> {
    let cmd_status = Command::new(cmd).args(args).status()?;

    match cmd_status.success() {
        true => Ok(()),
        false => Err(std::io::Error::other(
            format!("Ошибка в выполнении команды {}", cmd),
        )),
    }
}

#[tauri::command]
fn startlink(link: String) {
    let mut cmd = Command::new("xdg-open");
    cmd.arg(&link);

    match cmd.spawn() {
        Ok(..) => println!("Открываю ссылку: {}", &link),
        Err(e) => println!("Ошибка {e}")
    }
}

//Работа с темами
#[tauri::command]
fn theme_path(path: String) -> Result<String, String> {
    match fs::read_to_string(path) {
        Ok(content) => {
            println!("Подгружаем css стили.. Полет нормальный");
            Ok(content)
        }
        Err(e) => {
            eprintln!("Ошибка чтения CSS: {}", e);
            Ok(String::from(data::DATA))
        }
    }
}


#[tauri::command]
fn set_theme(name: String) -> Result<Config, String> {
    let mut config = config_read()?;
    config.theme = name;
    config_write(config)
}


#[tauri::command]
fn custom_theme_path() -> Vec<String> {
    let pathfolder = format!("{}/.config/enos_manager/",var("HOME").unwrap());
    let pathfile = format!("{}/.config/enos_manager/default.css",var("HOME").unwrap());
    let pathfolder = Path::new(&pathfolder);
    let pathfile = Path::new(&pathfile);
    let _null: Vec<String> = Vec::new();

    match pathfolder.exists() {
        true => {
            match pathfile.exists() {
                true => println!("Default file exist"),
                false => fs::write(pathfile, data::DATA).expect("Не получилось записать css файл"),
            }
            let theme_prefix = "css";
            let mut themes_list = Vec::new();
            let entries = match fs::read_dir(pathfolder) {
                Ok(dir) => dir,
                Err(_) => return Vec::new(),
            };
            for entry in entries.flatten() {
                let entry = entry;
                let path = entry.path();

                if path.is_file() {
                    if let Some(ext) = path.extension() {
                        if ext == theme_prefix {
                            if let Some(file_name) = path.file_name() {
                                themes_list.push(file_name.to_string_lossy().into_owned());
                            }
                        }
                    }
                }
            }
            themes_list
        }
        false => {
            let _ = fs::create_dir_all(pathfolder);
            fs::write(pathfile, data::DATA);

            Vec::new()
        }
    }
}




//Работа с языками
fn system_lang() -> String {
    let syslang = env::var("LANG").expect("Ошибка получения языка").to_string().chars().take(2).collect();
    syslang
}

#[tauri::command]
fn set_lang(lang: String) -> Result<Config, String> {
    let mut config = config_read()?;
    config.lang = lang;
    println!("Ставим язык: {}", config.lang);
    config_write(config)
}

#[tauri::command]
fn curr_lang() -> String {
    let config = config_read();
    config.unwrap().lang
}



//Работа с конфигами
#[tauri::command]
fn config_read() -> Result<Config, String> {
    let path = format!("{}/.config/enos_manager/settings.json",var("HOME").unwrap());
    let path2 = format!("{}/.config/enos_manager/",var("HOME").unwrap());
    let path = Path::new(&path);
    let path2 = Path::new(&path2);

    match path.exists() {
        true => {
            println!("Читаем конфиг");
            let file = File::open(path).map_err(|e| e.to_string())?;
            let reader = BufReader::new(file);
            let config: Config = serde_json::from_reader(reader)
            .map_err(|e| format!("Ошибка парсинга конфига: {}", e))?;
            Ok(config)
            }

        false => {
            println!("Ошибка чтения конфига, может он просто еще не создан?");
            fs::create_dir_all(path2).map_err(|e| e.to_string())?;

            let default_config = Config::default();
            let file = fs::File::create(path).map_err(|e| e.to_string())?;
            let mut writer = BufWriter::new(file);

            serde_json::to_writer_pretty(&mut writer, &default_config).map_err(|e| e.to_string())?;
            writer.flush().map_err(|e| e.to_string())?;

            Ok(default_config)
        }


    }
}

#[tauri::command]
fn config_write(config: Config) -> Result<Config, String> {
    println!("Запись в конфиг..");

    let path = format!("{}/.config/enos_manager/settings.json",var("HOME").unwrap());
    let path2 = format!("{}/.config/enos_manager/",var("HOME").unwrap());
    let path = Path::new(&path);
    let path2 = Path::new(&path2);

    fs::create_dir_all(path2).map_err(|e| format!("Ошибка создания директории для конфига: {}", e))?;

    let file = File::create(path)
    .map_err(|e| format!("Ошибка создания конфига: {}", e))?;
    let writer = BufWriter::new(file);

    serde_json::to_writer_pretty(writer, &config)
    .map_err(|e| format!("Ошибка записи конфига: {}", e))?;

    Ok(config)
}

#[tauri::command]
fn get_home_dir() -> String {
    env::var("HOME").unwrap_or_else(|_| "/home/user".to_string())
}


fn mbwayland() -> bool {
    let output = env::var("XDG_SESSION_TYPE").unwrap_or_default();

    match &*output {
        "wayland" => true,
        "x11" => false,
        _ => panic!("You dont have x11 or wayland"),
    }
}

fn main() {
    println!("Надеюсь, что вы не ИИ-фоб, и понимаете, что вся эта отладка написана вручную, просто, чтобы вы могли понять, на каком месте программа застряла. Удачного просмотра!");
    match mbwayland() {
        true => {
            set_var("XDG_RUNTIME_DIR", "/tmp/runtime-root");
            set_var("WEBKIT_DISABLE_COMPOSITING_MODE", "1");
            println!("Работаем с вяленым");
        }
        false => println!("Хорг запущен, еще посидим")
    }

    tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![startprog, theme_path, set_theme, custom_theme_path, config_read, config_write, get_home_dir, set_lang, curr_lang, startlink, tweak, check_prog])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
