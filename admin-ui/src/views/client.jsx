import { Box, IconButton, Paper } from "@mui/material";
import BottomNav from "../components/bottomNav";
import Tabs from '@mui/material/Tabs';
import Tab from '@mui/material/Tab';
import Typography from '@mui/material/Typography';
import React, { useState } from "react";
import AddIcon from '@mui/icons-material/Add';
import CloseIcon from '@mui/icons-material/Close';
import AddClientDialog from "../components/addClientDialog";
import ClientSession from "../components/clientSession";
import { useDispatch, useSelector } from "react-redux";
import { addFClient, addFClientCsMsg, addFClientScMsg, dropFClient, setCurrentClient } from "../features/clientSlice";
import clientMgrApi from "tauri-plugin-clientmgr-api";


function ClientTabs() {
    const [tab, setTab] = useState(0);
    const [clientList, setClientList] = useState([]);
    const [displayClient, setDisplayClient] = useState(null);
    const [showAddClient, setShowAddClient] = useState(false);
    const dispatch = useDispatch();

    const clientReducer = useSelector(state => state.client);

    function handleTabChange(_, newValue) {
        setTab(newValue);
        setDisplayClient(clientList[newValue]);
    }

    function handleCloseClientTab(idx) {
        let client = clientList[idx];
        console.log(`closing client ${client.id}`);
        clientList.splice(idx, 1);
        setClientList(clientList);
        if (clientList.length === 0) {
            setDisplayClient(null);
        } else {
            let len = clientList.length - 1;
            setDisplayClient(clientList[len]);
            setTab(len);
        }
        clientMgrApi.drop_fclient(Number(client.id)).then(res => {
            console.log(res)
        }).catch(err => {
            console.error(err)
        });
    }

    function addClient({ id, username, password, normalLogin }) {
        console.log(id, username, password, normalLogin);
        if (!normalLogin) {
            // check duplicated
            if (clientList.findIndex(c => c.id === id) !== -1) {
                console.error(`client-${id} exist`);
                return;
            }
            clientMgrApi.add_fclient(Number(id)).then(_ => {
                let client = { id, scMsg: [], csMsg: [] }
                clientList.push(client);
                setClientList(clientList);
                setDisplayClient(clientList[clientList.length - 1]);
            }).catch(err => {
                console.error(err);
            });
        }
        else{
            // TODO:
        }
        // close dialog
        setShowAddClient(false);
    }

    return (
        <Box sx={{ width: '100%' }}>
            <Box sx={{ borderBottom: 1, borderColor: 'divider' }}>
                <Tabs value={tab} onChange={handleTabChange} aria-label="basic tabs example">
                    {
                        clientList.map((client, idx) => <Tab key={idx} label={
                            <span>
                                {`Client-${client.id}`}
                                <IconButton size="small" onClick={handleCloseClientTab.bind(null, idx)}>
                                    <CloseIcon />
                                </IconButton>
                            </span>
                        } />)
                    }
                    <Tab onClick={setShowAddClient.bind(null, true)} icon={<AddIcon />} />
                </Tabs>
            </Box>
            {displayClient ? <ClientSession client={displayClient} /> : <Typography>Start by adding a new client</Typography>}
            <AddClientDialog show={showAddClient} setShow={setShowAddClient} onClickAdd={addClient} />
        </Box>
    );
}


function ViewClient() {

    return (
        <Box>
            <ClientTabs />
            <BottomNav />
        </Box>
    )
}

export default ViewClient;