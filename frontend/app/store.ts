// src/app/store.ts
import { configureStore } from "@reduxjs/toolkit";
import { ServerApi } from "@/app/services/ServerApi";

export const store = configureStore({
  reducer: {
    [ServerApi.reducerPath]: ServerApi.reducer,
  },
  middleware: (getDefaultMiddleware) =>
    getDefaultMiddleware().concat(ServerApi.middleware),
});

export type RootState = ReturnType<typeof store.getState>;
export type AppDispatch = typeof store.dispatch;
