use std::fs;
use std::fs::File;
use uuid::{Uuid};

#[tauri::command]
pub fn app_add_event(_item: &str, filepath: &str) -> String {
    let file = File::create(filepath);
    if file.is_err() {
        return "识别错误！".to_string();
    }

    let icon = windows_icons::get_icon_by_path(filepath);
    let mut icon_path = String::new();
    icon_path.push_str("E:\\1Temp\\icon-test\\");
    icon_path.push_str(Uuid::new_v4().to_string().as_str());
    icon_path.push_str(".png");

    println!("{:?}", icon_path);

    match icon.save(icon_path) {
        Ok(_) => "".to_string(),
        Err(_) => "识别错误".to_string()
    }
}

#[tauri::command]
pub fn load()->Vec<String>{
    let mut vec = Vec::new();
    match fs::read_dir("E:\\1Temp\\icon-test\\"){
        Ok(result)=>{
            for entry in result {
                let entry = entry.unwrap();
                let path = entry.path();
                vec.push(path.to_str().unwrap().to_string())
            }
            vec
        },
        Err(_)=>vec
    }
}

// struct Item{
//     path:String
// }