import { configureStore } from "@reduxjs/toolkit";
import { setupListeners } from "@reduxjs/toolkit/query";
import { templateApi } from "@/app/services/TemplateApi";
import { ServerApi } from "@/app/services/ServerApi";

export const store = configureStore({
  reducer: {
    [templateApi.reducerPath]: templateApi.reducer,
    [ServerApi.reducerPath]: ServerApi.reducer,
  },
  middleware: (getDefaultMiddleware) =>
    getDefaultMiddleware()
      .concat(templateApi.middleware)
      .concat(ServerApi.middleware),
});

setupListeners(store.dispatch);

export type RootState = ReturnType<typeof store.getState>;
export type AppDispatch = typeof store.dispatch;

export default store;
