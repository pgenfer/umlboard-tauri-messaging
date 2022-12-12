#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{collections::HashMap, string::String};
use serde::{Serialize, Deserialize, de::DeserializeOwned};
use serde_json::{json, Value};
use ts_rs::TS;
use std::string::ToString;
use strum_macros::Display;

#[tauri::command]
fn ipc_message(message: Action) -> Action {
    let (domain, r#type) = message.extract_domain_and_type();
    let action = Action {r#type: r#type.to_string(), payload: message.payload.clone()};
    let service = EditNameService{};
    let service_handler = |action| service.receive_action(domain, action);
    let mut handlers = HashMap::new();
    // we register an action handler for all actions of this domain
    // this would normally be registered during application startup
    handlers.insert(EDIT_NAME, service_handler);
    let handler_option = handlers.get(&*domain); 
    let response = match handler_option {
        Some(handler) => {
            let response_action = handler(action).unwrap();
            response_action
        }
        None => service.create_no_op()
    };
    response // response is an action converted to a JSON object
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![ipc_message])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[derive(Deserialize, Serialize)]
struct Action {
    r#type: String,
    payload: Option<Value>
}

impl Action {
    fn extract_domain_and_type(&self) -> (&str, &str) {
       let domain_and_type: Vec<&str> = self.r#type.split("/").collect();
       if domain_and_type.len() == 2 {
            let domain = domain_and_type[0];
            let r#type = domain_and_type[1];
            (domain, r#type)
       } else {
            // action could not be separated
            ("","")
       }
    }
}

#[derive(TS, Serialize, Deserialize)]
#[ts(export, rename_all="camelCase")]
#[ts(export_to = "../src/bindings/edit-name-dto.ts")]
#[serde(rename_all(deserialize="camelCase", serialize="camelCase"))]
struct EditNameDto {
    new_name: String
}

#[derive(Serialize, Deserialize, Display)]
#[serde(rename_all(serialize="camelCase", deserialize="camelCase"), tag = "type", content = "payload")]
#[strum(serialize_all = "camelCase")]
enum EditNameAction {
    ChangeName(EditNameDto),
    CancelNameChange,
    NameChanged(EditNameDto),
    NameChangeCanceled(EditNameDto),
    NameChangedError
}

// we need one file with constants, like
// this will also be generated for redux to use a slice name
pub const EDIT_NAME: &str = "editName";


trait ActionHandler {
    type TActionType: DeserializeOwned + Serialize + std::fmt::Display;

    /// TODO: remove this and simply return None
    fn create_no_op(&self) -> Action {
        Action { r#type: String::from("noOperation"), payload: None }
    }

    fn handle_action(&self, action: Self::TActionType) -> Result<Self::TActionType, serde_json::Error>;
    
    fn receive_action(&self, domain: &str, action: Action) -> Result<Action, serde_json::Error> {
        // convert action to json object (otherwise we don't know how to parse it into an enum)
        let json_action = json!({
            "type": action.r#type,
            "payload": action.payload
        });
        // convert payload to enum value
        let incoming: Self::TActionType = serde_json::from_value(json_action)?;
        let response = self.handle_action(incoming)?;
        let response_type = response.to_string();
        let response_json = serde_json::to_value(response)?;
        let mut response_action: Action = serde_json::from_value(response_json)?;
        response_action.r#type = format!("{}/{}", domain, response_type);
        Ok(response_action)
    }
}

struct EditNameService {}

impl ActionHandler for EditNameService {
    type TActionType = EditNameAction;

    fn handle_action(&self, action: Self::TActionType) -> Result<Self::TActionType, serde_json::Error> {
        let response = match action {
            EditNameAction::ChangeName(data) => EditNameAction::NameChanged(data),
            EditNameAction::CancelNameChange => EditNameAction::NameChangeCanceled(EditNameDto { new_name: "UMLBoard".to_string() }),
            _ => EditNameAction::NameChangedError
        };
        Ok(response)
    }
}