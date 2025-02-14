import { configureStore } from "@reduxjs/toolkit";
import { setupListeners } from "@reduxjs/toolkit/query";
import { templateApi } from "@/app/services/TemplateApi";
import { ServerApi } from "@/app/services/ServerApi";
import { contactApi } from "@/app/services/ContactApi";
import { listApi } from "@/app/services/ListApi";
import { campaignApi } from '@/app/services/CampaignApi';

export const store = configureStore({
  reducer: {
    [templateApi.reducerPath]: templateApi.reducer,
    [ServerApi.reducerPath]: ServerApi.reducer,
    [contactApi.reducerPath]: contactApi.reducer,
    [listApi.reducerPath]: listApi.reducer,
    [campaignApi.reducerPath]: campaignApi.reducer,
  },
  middleware: (getDefaultMiddleware) =>
    getDefaultMiddleware()
      .concat(templateApi.middleware)
      .concat(ServerApi.middleware)
      .concat(listApi.middleware)
      .concat(contactApi.middleware)
      .concat(campaignApi.middleware),
});

setupListeners(store.dispatch);

export type RootState = ReturnType<typeof store.getState>;
export type AppDispatch = typeof store.dispatch;

export default store;
