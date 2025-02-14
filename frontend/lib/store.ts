import { configureStore } from "@reduxjs/toolkit";
import { setupListeners } from "@reduxjs/toolkit/query";
import { templateApi } from "@/app/services/TemplateApi";
import { ServerApi } from "@/app/services/ServerApi";
import { contactApi } from "@/app/services/ContactApi";
import { listApi } from "@/app/services/ListApi";
import { campaignApi } from '@/app/services/CampaignApi';
<<<<<<< HEAD
import { mailApi } from '@/app/services/MailApi';
=======
>>>>>>> 9a97571 (WIP campaign)

export const store = configureStore({
  reducer: {
    [templateApi.reducerPath]: templateApi.reducer,
    [ServerApi.reducerPath]: ServerApi.reducer,
    [contactApi.reducerPath]: contactApi.reducer,
    [listApi.reducerPath]: listApi.reducer,
    [campaignApi.reducerPath]: campaignApi.reducer,
<<<<<<< HEAD
    [mailApi.reducerPath]: mailApi.reducer,
=======
>>>>>>> 9a97571 (WIP campaign)
  },
  middleware: (getDefaultMiddleware) =>
    getDefaultMiddleware()
      .concat(templateApi.middleware)
      .concat(ServerApi.middleware)
      .concat(listApi.middleware)
      .concat(contactApi.middleware)
<<<<<<< HEAD
      .concat(campaignApi.middleware)
      .concat(mailApi.middleware)
=======
      .concat(campaignApi.middleware),
>>>>>>> 9a97571 (WIP campaign)
});

setupListeners(store.dispatch);

export type RootState = ReturnType<typeof store.getState>;
export type AppDispatch = typeof store.dispatch;

export default store;
