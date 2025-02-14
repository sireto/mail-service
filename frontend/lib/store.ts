import { configureStore } from '@reduxjs/toolkit';
import { setupListeners } from '@reduxjs/toolkit/query';
import { templateApi } from '@/app/services/TemplateApi';
import { listApi } from '@/app/services/ListApi';
import { campaignApi } from '@/app/services/CampaignApi';


const store = configureStore({
    reducer: {
        // auto-generate the reducer...
        [templateApi.reducerPath]: templateApi.reducer,
        [listApi.reducerPath]: listApi.reducer,
        [campaignApi.reducerPath]: campaignApi.reducer,
    },
    middleware: (getDefaultMiddleware) => getDefaultMiddleware()
        .concat(templateApi.middleware)
        .concat(listApi.middleware)
        .concat(campaignApi.middleware),
});

setupListeners(store.dispatch);

export type RootState = ReturnType<typeof store.getState>;
export type AppDispatch = typeof store.dispatch;

export default store;