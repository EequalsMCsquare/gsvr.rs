import { BottomNavigation, BottomNavigationAction, Paper, } from "@mui/material";
import { setLocation } from '../features/menuSlice';
import { useSelector, useDispatch } from 'react-redux';
import { Link } from "react-router-dom";
import SpeedIcon from '@mui/icons-material/Speed';
import PeopleAltIcon from '@mui/icons-material/PeopleAlt';
import SettingsIcon from "@mui/icons-material/Settings"

function BottomNav() {
    const location = useSelector((state) => state.menu.location);
    const dispatch = useDispatch();
    return (
        <Paper style={{ position: 'fixed', bottom: 0, left: 0, right: 0 }} elevation={18}>
            <BottomNavigation
                showLabels
                value={location}
                onChange={(_, newValue) => {
                    dispatch(setLocation(newValue));
                }}
            >
                <BottomNavigationAction component={Link} to="/clients" label="Client" icon={<PeopleAltIcon/>} />
                <BottomNavigationAction component={Link} to="/benchmark" label="Bench" icon={<SpeedIcon />} />
                <BottomNavigationAction component={Link} to="/setting" label="Setting" icon={<SettingsIcon />} />
            </BottomNavigation>
        </Paper>
    );
}

export default BottomNav;
