use std::{env, path::Path, fs, io::stdin};
use walkdir::WalkDir;
use encoding_rs::{BIG5};
use chardetng::{self, EncodingDetector};

fn is_translate_file(path: &Path) -> bool{
    path.is_file() && path.to_str().unwrap().ends_with("_CH.txt")
}

fn convert_encoding(path: &Path) {
    let content = fs::read(path).expect("read content from file");
    let mut detector = EncodingDetector::new();
    detector.feed(&content, true);
    let encoding = detector.guess(None, true);
    if encoding == BIG5 {
        let (str, _, _) = BIG5.decode(&content);
        fs::write(path, str.as_bytes()).expect("write UTF8 content to file");
        println!("轉換編碼BIG5 to UTF8: {}", path.to_str().unwrap());
    } else {
        println!("{:?}: {}", encoding, path.to_str().unwrap());
    }
}

fn main() {
    let location = {
        match env::args().nth(1) {
            Some(s) => s,
            None => {
                println!("請輸入Project Zomboid資料夾路徑:");
                let mut s = String::new();
                stdin().read_line(&mut s).expect("input Project Zomboid path");
                s.trim_end().to_owned()
            },
        }
    };
    /*
    SteamLibrary\steamapps\common\ProjectZomboid
    SteamLibrary\steamapps\common\ProjectZomboid\media\lua\shared\Translate\CH\XXX_CH.txt
    SteamLibrary\steamapps\workshop\content\108600
    SteamLibrary\steamapps\workshop\content\108600\1299328280\mods\XXXX\media\lua\shared\Translate\CH
     */
    let main_path = Path::new(&location);
    let main_translate_path = main_path.join("media\\lua\\shared\\Translate\\CH");
    if !main_translate_path.exists() {
        panic!("無法找到主程式翻譯檔案，請確認路徑。")
    }
    println!("開始搜尋主程式翻譯檔案 {}", main_translate_path.to_string_lossy());
    for entry in WalkDir::new(main_translate_path) {
        if let Ok(entry) = entry {
            if is_translate_file(entry.path()) {
                convert_encoding(entry.path());
            }
        }
    }

    let workshop_path = main_path.parent().unwrap().parent().unwrap().join("workshop\\content\\108600");
    if !workshop_path.exists() {
        panic!("無法找到工作坊資料夾");
    }
    println!("開始搜尋工作坊翻譯檔案 {}", workshop_path.to_string_lossy());
    for entry in WalkDir::new(workshop_path) {
        if let Ok(entry) = entry {
            if is_translate_file(entry.path()) {
                convert_encoding(entry.path());
            }
        }
    }
}
