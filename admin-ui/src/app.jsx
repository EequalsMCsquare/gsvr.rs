import { useDispatch } from 'react-redux'
import Router from "./router"
import React, { useEffect } from "react";
import clientMgrApi from "tauri-plugin-clientmgr-api";
import { useState } from 'react';
import { addFClientScMsg } from "./features/clientSlice";

function App() {
    const [funlisten, setFunlisten] = useState(() => { });
    const dispatch = useDispatch();

    useEffect(() => {
        clientMgrApi.fclient_listen_reply((msg) => {
            console.log(`recv reply: ` + msg);
            dispatch(addFClientScMsg({
                id: msg.payload.id,
                msg: {
                    name: "asd",
                    content: msg.payload.payload
                }
            }))
            console.log(msg);
        }).then(res => {
            setFunlisten(() => {
                console.log("unlisten fclient reply");
                res();
            });
        }).catch(err => {
            console.error(err);
        });
        return funlisten;
    }, [clientMgrApi]);

    return (
        <React.StrictMode>
            <Router />
        </React.StrictMode>
    )
}

export default App;