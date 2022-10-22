import { createSlice } from '@reduxjs/toolkit'

export const menuSlice = createSlice({
    name: 'menuLocation',
    initialState: {
        location: 0
    },
    reducers: {
        setLocation: (state, action) => {
            state.location = action.payload
        }
    }
});

export const { setLocation } = menuSlice.actions;
export default menuSlice.reducer;