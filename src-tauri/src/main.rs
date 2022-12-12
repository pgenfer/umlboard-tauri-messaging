#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use core::panic;
use std::{collections::HashMap, string::String};
use serde::{Serialize, Deserialize, de::DeserializeOwned};
use serde_json::{json, Map, Value};
use ts_rs::TS;

#[tauri::command]
fn ipc_message(message: Action) -> Value {
    let service = EditNameService::new();
    let service_handler = |json| service.receive_action(json);
    let mut handlers = HashMap::new();
    // we register an action handler for all actions of this domain
    // this would normally be registered during application startup
    handlers.insert(EDIT_NAME, service_handler);
    let domain = message.domain;
    let handler_option = handlers.get(&*domain); 
    let response = match handler_option {
        Some(handler) => handler(message.action),
        // TODO: we should write a warning here and send a NoOp back to the webview
        None => panic!()
    };
    response // response is an action converted to a JSON object
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![ipc_message])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

/// processes the incoming action and generates response action.
/// The incoming action is a json object and will be converted to an Enum
/// The response is an option of the same enum converted into a json object
/// TODO: this function should take an Enum and return an Enum, maybe split into a generic and non generic method?
// fn handle_edit_name(action_json: Value) -> Value {

//     // maybe we can outsource this to a generic method
//     let incoming: Result<EditNameAction, serde_json::Error> = serde_json::from_value(action_json);
//     let result = match incoming {
//         Ok(action) => {
//             match action {
//                 EditNameAction::ChangeName(payload) => {
//                     EditNameAction::NameChanged(payload)
//                 },
//                 EditNameAction::CancelNameChange => {
//                     let dto = EditNameDto {
//                         new_name: String::from("UMLBoard")
//                     };
//                     EditNameAction::NameChangeCanceled(dto)
//                 },
//                 _ => EditNameAction::NameChangedError
//             }
//         },
//         Err(error) => {
//             EditNameAction::NameChangedError
//         }
//     };
//     let result_value = match action_to_json(EDIT_NAME.to_owned(), result) {
//         Some(action_string) => action_string,
//         None => panic!()
//     };
//     result_value
// }

/// takes an action and converts it to a json object.
/// Replaces the action's type with <domain>/<type> to match Redux' action type
/// TODO: maybe we can also make this a bit more general?
// fn action_to_json(domain: String, action: EditNameAction) -> Option<Value> {
//     // TODO: this works but looks incredible ugly, check how to improve this
//     let error_action = EditNameAction::NameChangedError;
//     let error_json = serde_json::to_value(error_action).unwrap();
//     let result = serde_json::to_value(action);
//     let json_result = match result {
//         Ok(mut json) => {
//             let key = String::from("type");
//             let json_object: &mut  Map<String, Value>  = json.as_object_mut()?;
//             let action_type = &json_object.get(&key)?.as_str()?;
//             let domain_and_action = format!("{}/{}", domain, action_type);
//             json_object.entry(key).and_modify(|e| *e = json!(domain_and_action));
//             // json_object[*action_type] = serde_json::Value::String(domain_and_action);
//             json
//             // serde_json::to_string(json_object).unwrap()
//         },
//         Err(_) => error_json
//     };
//     Some(json_result)
// }

#[derive(Serialize, Deserialize, Debug)]
struct Action {
    domain: String,
    action: Value
}

#[derive(TS, Serialize, Deserialize)]
#[ts(export, rename_all="camelCase")]
#[ts(export_to = "../src/bindings/edit-name-dto.ts")]
#[serde(rename_all(deserialize="camelCase"))]
#[serde(rename_all(serialize="camelCase"))]
struct EditNameDto {
    new_name: String
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all(deserialize="camelCase"))]
#[serde(rename_all(serialize="camelCase"), tag = "type", content = "payload")]
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
    type TActionType: DeserializeOwned + Serialize;

    fn create_no_op(&self) -> Value {
        let value = json!({
            "type": "noOperation/noOperation",
            "payload": {}
        });
        value
    }

    fn action_to_json(&self, domain: String, action: Self::TActionType) -> Option<Value> {
        let result = serde_json::to_value(action);
        let json_result = match result {
            Ok(mut json) => {
                let key = String::from("type");
                let json_object: &mut  Map<String, Value>  = json.as_object_mut()?;
                let action_type = &json_object.get(&key)?.as_str()?;
                let domain_and_action = format!("{}/{}", domain, action_type);
                json_object.entry(key).and_modify(|e| *e = json!(domain_and_action));
                // json_object[*action_type] = serde_json::Value::String(domain_and_action);
                json
                // serde_json::to_string(json_object).unwrap()
            },
            Err(_) => self.create_no_op()
        };
        Some(json_result)
    }

    fn handle_action(&self, action: Self::TActionType) -> Self::TActionType;
    
    fn receive_action(&self, json_action: Value) -> Value {
        let incoming: Result<Self::TActionType, serde_json::Error> = serde_json::from_value(json_action);
        let result = match incoming {
            Ok(action) => {
                let response = self.handle_action(action);
                let response_action = self.action_to_json(EDIT_NAME.to_string(), response).unwrap_or(self.create_no_op());
                response_action
            },
            Err(error) => {
                // error when converting action from json to object -> should not happen in production
                // TODO: we should print log and return NoOp action here
                let value = self.create_no_op();
                value
            }
        };
        result

    }
}

struct EditNameService {}

impl EditNameService {
    fn new() -> Self { Self {  } }
}

impl ActionHandler for EditNameService {
    type TActionType = EditNameAction;

    fn handle_action(&self, action: Self::TActionType) -> Self::TActionType {
        let response = match action {
            EditNameAction::ChangeName(data) => EditNameAction::NameChanged(data),
            EditNameAction::CancelNameChange => EditNameAction::NameChangeCanceled(EditNameDto { new_name: "UMLBoard".to_string() }),
            _ => EditNameAction::NameChangedError
        };
        response
    }
}