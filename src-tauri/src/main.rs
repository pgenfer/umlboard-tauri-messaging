#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{string::String, collections::HashMap};
use serde::{Serialize, Deserialize, de::DeserializeOwned};
use serde_json::{Value};
use ts_rs::TS;
use std::string::ToString;
use strum_macros::Display;

#[tauri::command]
fn ipc_message(message: IpcMessage) -> IpcMessage {
    // Normally, we would have some kind of dictionary 
    // with our services created during startup.
    // In this example, we just create everything in place here for simplifaction
    let service = ClassifierService{};
    let mut handlers = HashMap::new();
    handlers.insert(service.domain(), service);
    
    // this is were our actual command begins
    let message_handler = handlers.get(&*message.domain).unwrap(); 
    let response = message_handler.receive_action(message.action).unwrap();
    IpcMessage {
        domain: message_handler.domain(),
        action: response
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![ipc_message])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[derive(Deserialize, Serialize)]
struct IpcMessage {
    domain: String,
    action: Value
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
enum ClassifierAction {
    RenameClassifier(EditNameDto),
    CancelClassifierRename,
    ClassifierRenamed(EditNameDto),
    ClassifierRenameCanceled(EditNameDto),
    ClassifierRenameError
}

// we need one file with constants, like
// this will also be generated for redux to use a slice name
pub const CLASSIFIER_DOMAIN: &str = "classifier";

trait ActionHandler {
    type TActionType: DeserializeOwned + Serialize + std::fmt::Display;

    fn domain(&self) -> String;
    fn handle_action(&self, action: Self::TActionType) -> Result<Self::TActionType, serde_json::Error>;    
    fn receive_action(&self, json_action: Value) -> Result<Value, serde_json::Error> {
        // convert json to action
        let incoming: Self::TActionType = serde_json::from_value(json_action)?;
        // call action specific handler
        let response = self.handle_action(incoming)?;
        // convert response to json
        let response_json = serde_json::to_value(response)?;
        Ok(response_json)
    }
}

struct ClassifierService {}
impl ClassifierService {
    pub fn update_classifier_name(&self, new_name: &str) -> () {/* TODO: implement */}
}
impl ActionHandler for ClassifierService {
    type TActionType = ClassifierAction;

    fn domain(&self) -> String { CLASSIFIER_DOMAIN.to_string()}
    fn handle_action(&self, action: Self::TActionType) -> Result<Self::TActionType, serde_json::Error> {
        let response = match action {
            ClassifierAction::RenameClassifier(data) => {
                self.update_classifier_name(&data.new_name);
                ClassifierAction::ClassifierRenamed(data)
            },
            ClassifierAction::CancelClassifierRename =>
                ClassifierAction::ClassifierRenameCanceled(
                    EditNameDto { new_name: "Old Classname".to_string() }
                )
            ,
            _ => ClassifierAction::ClassifierRenameError
        };
        Ok(response)
    }
}