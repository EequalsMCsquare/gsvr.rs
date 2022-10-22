import { configureStore } from '@reduxjs/toolkit';
import menuReducer from './menuSlice';
import clientReducer from './clientSlice';

export const store = configureStore({
  reducer: {
    menu: menuReducer,
    client: clientReducer,
  },
})