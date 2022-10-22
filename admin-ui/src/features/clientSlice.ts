import { createSlice } from "@reduxjs/toolkit";

export interface message {
    name: string,
    content: string,
}

export interface fclient {
    id: number,
    csMsg: message[],
    scMsg: message[]
}

export interface nclient {
    username: string,
    csMsg: string[],
    scMsg: string[]
}

export const clientSlice = createSlice({
    name: 'clientMgr',
    initialState: {
        fclients: new Map<number, fclient>(),
        nClients: new Map<string, nclient>(),
        currentClient: null,
    },
    reducers: {
        addFClient: (state, action) => {
            state.fclients.set(action.payload.id, {
                id: action.payload.id,
                csMsg: Array<message>(),
                scMsg: Array<message>(),
            })
        },
        // drop by index
        dropFClient: (state, action) => {
            state.fclients.delete(action.payload.id);
        },
        /*
            payload => {
                id: Number, 
                msg: {
                    name: String, 
                    content: String,
                }
            }
        */
        addFClientCsMsg: (state, action) => {
            let client = state.fclients.get(action.payload.id);
            if (client) {
                client.csMsg.push(action.payload.msg);
                return;
            }
            console.error(`fclient-${action.payload.id} not found`);
        },
        addFClientScMsg: (state, action) => {
            let client = state.fclients.get(action.payload.id);
            if (client) {
                client.scMsg.push(action.payload.msg);
                console.log(client.scMsg);
                return;
            }
            console.error(`fclient-${action.payload.id} not found`);
        },
        /*
            payload => {
                isNClient: bool, 
                id: Number, 
                username: String,
            }
        */
        setCurrentClient: (state, action) => {
            if (action.payload.isNClient) {
                let client = state.nClients.get(action.payload.username);
                if (client) {
                    state.currentClient = client as any;
                    return;
                }
                console.error(`nclient-${action.payload.username} not found`);
            } else {
                let client = state.fclients.get(action.payload.id);
                if (client) {
                    state.currentClient = client as any;
                    return;
                }
                console.error(`fclient-${action.payload.id} not found`);
            }
            console.error
        }
    }
});

export const { addFClient, dropFClient, addFClientCsMsg, addFClientScMsg, setCurrentClient } = clientSlice.actions;
export default clientSlice.reducer;