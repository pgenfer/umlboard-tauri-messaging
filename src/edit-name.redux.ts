import { createSlice, PayloadAction } from "@reduxjs/toolkit";
import { EditNameDto } from "./bindings/edit-name-dto";

type State = {
    currentName: string
}

const initialState: State = {
    currentName: 'UMLBoard'
}

const editNameSlice = createSlice({
    name: 'classifier',
    initialState,
    reducers: {
        renamingClassifier(state, action: PayloadAction<EditNameDto>) {
            state.currentName = action.payload.newName;
        },
        renameClassifier(state, action: PayloadAction<EditNameDto>) {
            // handled by backend
        },
        cancelClassifierRename(state) {
            // handled by backend            
        },
        classifierRenamed(state, action: PayloadAction<EditNameDto>) {
            console.log('name changed called with: ' + action.payload.newName);
            // actually not necessary, but keep here
            state.currentName = action.payload.newName;
        },
        classifierRenameCanceled(state, action: PayloadAction<EditNameDto>) {
            // restore the name from core process
            state.currentName = action.payload.newName;
        }
    }
});

export const editNameReducer = editNameSlice.reducer;
export const {renamingClassifier, renameClassifier, cancelClassifierRename} = editNameSlice.actions;