import { configureStore } from "@reduxjs/toolkit";
import { setupListeners } from "@reduxjs/toolkit/query";
import { templateApi } from "@/app/services/TemplateApi";
import { ServerApi } from "@/app/services/ServerApi";
import { contactApi } from "@/app/services/ContactApi";
import { listApi } from "@/app/services/ListApi";
import { campaignApi } from '@/app/services/CampaignApi';
<<<<<<< HEAD
<<<<<<< HEAD
import { mailApi } from '@/app/services/MailApi';
=======
>>>>>>> 9a97571 (WIP campaign)
=======
import { mailApi } from '@/app/services/MailApi';
>>>>>>> 0b443a3 (feat: add update and analytics to the campaigns page)

export const store = configureStore({
  reducer: {
    [templateApi.reducerPath]: templateApi.reducer,
    [ServerApi.reducerPath]: ServerApi.reducer,
    [contactApi.reducerPath]: contactApi.reducer,
    [listApi.reducerPath]: listApi.reducer,
    [campaignApi.reducerPath]: campaignApi.reducer,
<<<<<<< HEAD
<<<<<<< HEAD
    [mailApi.reducerPath]: mailApi.reducer,
=======
>>>>>>> 9a97571 (WIP campaign)
=======
    [mailApi.reducerPath]: mailApi.reducer,
>>>>>>> 0b443a3 (feat: add update and analytics to the campaigns page)
  },
  middleware: (getDefaultMiddleware) =>
    getDefaultMiddleware()
      .concat(templateApi.middleware)
      .concat(ServerApi.middleware)
      .concat(listApi.middleware)
      .concat(contactApi.middleware)
<<<<<<< HEAD
<<<<<<< HEAD
      .concat(campaignApi.middleware)
      .concat(mailApi.middleware)
=======
      .concat(campaignApi.middleware),
>>>>>>> 9a97571 (WIP campaign)
=======
      .concat(campaignApi.middleware)
      .concat(mailApi.middleware)
>>>>>>> 0b443a3 (feat: add update and analytics to the campaigns page)
});

setupListeners(store.dispatch);

export type RootState = ReturnType<typeof store.getState>;
export type AppDispatch = typeof store.dispatch;

export default store;
