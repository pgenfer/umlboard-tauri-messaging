import { createSlice, PayloadAction } from "@reduxjs/toolkit";
import { EditNameDto } from "./bindings/edit-name-dto";

type State = {
    currentName: string
}

const initialState: State = {
    currentName: 'UMLBoard'
}

const editNameSlice = createSlice({
    name: 'editName',
    initialState,
    reducers: {
        changingName(state, action: PayloadAction<EditNameDto>) {
            state.currentName = action.payload.newName;
        },
        changeName(state, action: PayloadAction<EditNameDto>) {
            // TODO: send to backend
            state.currentName = action.payload.newName;
        },
        cancelNameChange(state) {
            // TODO: send to backend
            state.currentName = 'UMLBoard';
        },
        nameChanged(state, action: PayloadAction<EditNameDto>) {
            console.log('name changed called with: ' + action.payload.newName);
            // actually not necessary, but keep here
            state.currentName = action.payload.newName;
        },
        nameChangeCanceled(state, action: PayloadAction<EditNameDto>) {
            // restore the name from core process
            state.currentName = action.payload.newName;
        }
    }
});

export const editNameReducer = editNameSlice.reducer;
export const {changeName, cancelNameChange, changingName} = editNameSlice.actions;