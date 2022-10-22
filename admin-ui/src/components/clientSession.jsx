import { Autocomplete, Avatar, Button, Grid, IconButton, ListItem, ListItemAvatar, ListItemText, Paper, Stack, TextField, Typography } from "@mui/material";
import React, { useCallback, useState } from "react";
import { useEffect } from "react";
import hintService from "../service/hint";
import { ContentCopy as ContentCopyIcon } from "@mui/icons-material";
import { invoke } from "@tauri-apps/api/tauri";
import clientMgrApi from "tauri-plugin-clientmgr-api"

const csMock = [
    {
        name: "CsEcho",
        payload: 
`{
    "CsEcho": {
        "content": "hello world"
    }
}`
    },
    {
        name: "CsEcho",
        payload: 
`{
    "CsEcho": {
        "content": "hello world"
    }
}`
    },
    {
        name: "CsEcho",
        payload: 
`{
    "CsEcho": {
        "content": "hello world"
    }
}`
    },
    {
        name: "CsEcho",
        payload: 
`{
    "CsEcho": {
        "content": "hello world"
    }
}`
    },
    {
        name: "CsEcho",
        payload: 
`{
    "CsEcho": {
        "content": "hello world"
    }
}`
    },
]

const scMock = [
    {
        name: "ScEcho",
        payload:
`{
    "ScEcho": {
        "content": "you said: hello world"
    }
}`
    },
    {
        name: "ScEcho",
        payload:
`{
    "ScEcho": {
        "content": "you said: hello world"
    }
}`
    },
    {
        name: "ScEcho",
        payload:
`{
    "ScEcho": {
        "content": "you said: hello world"
    }
}`
    },
    {
        name: "ScEcho",
        payload:
`{
    "ScEcho": {
        "content": "you said: hello world"
    }
}`
    },
    {
        name: "ScEcho",
        payload:
`{
    "ScEcho": {
        "content": "you said: hello world"
    }
}`
    },
]
function Message(props) {
    const type = props.type; // cs or sc
    const idx = props.idx;
    const pb = props.pb;

    return (
        <Paper elevation={4} style={{ marginLeft: 5, marginRight: 5 }}>
            <ListItem
                secondaryAction={
                    <IconButton edge="end">
                        <ContentCopyIcon />
                    </IconButton>
                }
            >
                <ListItemAvatar>
                    <Avatar>{idx+1}</Avatar>
                </ListItemAvatar>
                <ListItemText
                    primary={pb.name}
                    secondary={
                        <TextField
                            fullWidth
                            multiline
                            maxRows={8}
                            size="large"
                            contentEditable={false}
                            color="secondary"
                            value={pb.payload}
                        />
                    }
                />
            </ListItem>
        </Paper>
    )
}

function AckHistory(props) {
    const scMsg = props.scmsg;

    return (
        <Stack paddingBottom={2} paddingTop={2} overflow={"scroll"} spacing={1} {...props}>
            {scMsg.map((e, i)=><Message key={i} idx={i} pb={e}/>)}
        </Stack>
    )
}

function ReqHistory(props) {
    const csMsg = props.csmsg;
    return (
        <Stack paddingBottom={2} paddingTop={2} overflow={"scroll"} spacing={1} {...props}>
            {csMsg.map((e, i)=><Message key={i} idx={i} pb={e}/>)}
        </Stack>
    )
}

function MessageInput(props) {
    const handleSendRequest = props.handleSendRequest;
    const [value, setValue] = useState("");
    const [inputValue, setInputValue] = useState("");
    const [hintOn, setHintOn] = useState(true);
    function handleValueChange(_, newVal) {
        if (hintOn) {
            hintService.tryHint({ key: newVal }).then(res => {
                setValue(res);
                setInputValue(res);
                setHintOn(false);
            }).catch(err => {
                console.error(err);
            });
        }
    }

    function handleInputChange(_, newVal) {
        setInputValue(newVal);
        setValue(newVal);
        if (newVal.length < 4) {
            setHintOn(true);
        }
    }

    return (
        <>
            <Autocomplete
                options={props.hintKeys}
                autoHighlight
                freeSolo
                clearOnEscape
                value={value}
                onChange={handleValueChange}
                inputValue={inputValue}
                onInputChange={handleInputChange}
                renderInput={
                    (params) =>
                        <TextField
                            {...params}
                            multiline
                            rows={7}
                        />
                }
            />
            <Button
                size="large"
                variant="contained" 
                color="success" 
                type="submit" 
                onClick={handleSendRequest.bind(null, value)}
            >Send</Button>
        </>
    )
}


function ClientSession(props) {
    const client = props.client;
    const [windowSize, setWindowSize] = useState({
        width: document.documentElement.clientWidth,
        height: document.documentElement.clientHeight,
    });
    const [hintKeys, setHintKeys] = useState([]);
    const onResize = useCallback(() => {
        setWindowSize({
            width: document.documentElement.clientWidth,
            height: document.documentElement.clientHeight,
        });
    }, []);
    // listen window resize event
    useEffect(() => {
        window.addEventListener('resize', onResize);
        return () => {
            window.removeEventListener('resize', onResize);
        }
    }, [onResize]);
    // init hint keys
    useEffect(() => {
        hintService.getOptions().then(res => {
            console.log(`hint keys: ${res}`);
            setHintKeys(res);
        }).catch(err => {
            console.error(err);
        });
    }, [hintService.getHintKeys]);

    function handleSendRequest(content) {
        clientMgrApi.fclient_request(Number(client.id), content).then(res => {
            console.log(res);
        }).catch(err => {
            console.error(err);
        });
    }

    return (
        <Grid container spacing={4}>
            <Grid item xs={6} md={6} lg={6}>
                <AckHistory
                    overflow='scroll'
                    maxHeight={windowSize.height - 160}
                    spacing={2}
                    mt={1}
                    mb={1}
                    scmsg={scMock}
                />
            </Grid>
            <Grid item xs={6} md={6}>
                <Stack
                    maxHeight={windowSize.height - 160}
                    spacing={2}
                >
                    <ReqHistory
                        overflow='scroll'
                        spacing={2}
                        mt={1}
                        mb={1}
                        csmsg={csMock}
                    />
                    <MessageInput 
                        hintKeys={hintKeys} 
                        handleSendRequest={handleSendRequest}
                        />
                </Stack>
            </Grid>
        </Grid>
    );
}
export default ClientSession;