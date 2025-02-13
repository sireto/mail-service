import { configureStore } from "@reduxjs/toolkit";
import { setupListeners } from "@reduxjs/toolkit/query";
import { templateApi } from "@/app/services/TemplateApi";
import { ServerApi } from "@/app/services/ServerApi";
import { listApi } from '@/app/services/ListApi';

export const store = configureStore({
  reducer: {
    [templateApi.reducerPath]: templateApi.reducer,
    [ServerApi.reducerPath]: ServerApi.reducer,
    [listApi.reducerPath]: listApi.reducer,
  },
  middleware: (getDefaultMiddleware) =>
    getDefaultMiddleware()
      .concat(templateApi.middleware)
      .concat(ServerApi.middleware)
      .concat(listApi.middleware),
});

setupListeners(store.dispatch);

export type RootState = ReturnType<typeof store.getState>;
export type AppDispatch = typeof store.dispatch;

export default store;
