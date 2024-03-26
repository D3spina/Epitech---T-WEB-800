use serde::Serialize;
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#[cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


#[derive(Serialize)]
struct Restaurant {
  nom: String,
  adresse: String,
  cuisine: String,
}

#[tauri::command]
fn get_data() -> Vec<Restaurant> {
  vec![
    Restaurant {
      nom: "Chez Antoine".into(),
      adresse: "123 rue de la République, Paris".into(),
      cuisine: "Française".into(),
    },
    Restaurant {
      nom: "La Bella Napoli".into(),
      adresse: "456 avenue de l'Opéra, Marseille".into(),
      cuisine: "Italienne".into(),
    },
    Restaurant {
      nom: "Le Dragon d'Or".into(),
      adresse: "789 boulevard Voltaire, Lyon".into(),
      cuisine: "Chinoise".into(),
    },
  ]
}


fn main() {
  tauri::Builder::default()
      .invoke_handler(tauri::generate_handler![get_data])
      .run(tauri::generate_context!())
    .expect("error while running tauri application");
}


