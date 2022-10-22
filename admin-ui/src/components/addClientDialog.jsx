import { Button, Dialog, DialogActions, DialogContent, DialogContentText, DialogTitle, FormControlLabel, Switch, TextField } from "@mui/material";
import { useState } from "react";
import PropTypes from 'prop-types';

function AddClientDialog(props) {
    const vis = props.show;
    const setVis = props.setShow;
    const onClickAdd = props.onClickAdd;

    const [clientId, setClientId] = useState("");
    const [username, setUsername] = useState("");
    const [password, setPassword] = useState("");
    const [normalLogin, setNormalLogin] = useState(false);

    function handleDialogClose() {
        setVis(false);
        setClientId("");
        setUsername("");
        setPassword("");
        setNormalLogin(false);
    }

    return (
        <Dialog fullWidth maxWidth="sm" open={vis} onClose={handleDialogClose}>
            <DialogTitle>Add Client</DialogTitle>
            <DialogContent>
                <DialogContentText>
                    Add a new client to the tab
                </DialogContentText>
                <FormControlLabel
                    control={<Switch size="medium" value={normalLogin} onChange={e => setNormalLogin(e.target.checked)} />}
                label="Normal Login" />
                {
                    normalLogin ?
                        <>
                            <TextField
                                autoFocus
                                margin="dense"
                                label="Username"
                                type="text"
                                fullWidth
                                variant="standard"
                                value={username}
                                onChange={e => setUsername(e.target.value)}
                            />
                            <TextField
                                margin="dense"
                                label="Password"
                                type="password"
                                fullWidth
                                variant="standard"
                                value={password}
                                onChange={e => setPassword(e.target.value)}
                            />
                        </> :
                        <TextField
                            autoFocus
                            margin="dense"
                            label="Player ID"
                            type="text"
                            fullWidth
                            variant="standard"
                            value={clientId}
                            onChange={e => setClientId(e.target.value)}
                        />
                }
            </DialogContent>
            <DialogActions>
                <Button onClick={handleDialogClose} variant="contained">Cancel</Button>
                <Button onClick={onClickAdd.bind(null, { id: clientId, username, password, normalLogin })} variant="contained" color="success">Add</Button>
            </DialogActions>
        </Dialog>
    )
}

AddClientDialog.prototype = {
    show: PropTypes.bool.isRequired,
    setShow: PropTypes.func.isRequired,
    onClickAdd: PropTypes.func.isRequired
}
export default AddClientDialog;