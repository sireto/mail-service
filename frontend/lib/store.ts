import { configureStore } from '@reduxjs/toolkit';
import { setupListeners } from '@reduxjs/toolkit/query';
import { templateApi } from '@/app/services/TemplateApi';


const store = configureStore({
    reducer: {
        // auto-generate the reducer...
        [templateApi.reducerPath]: templateApi.reducer,
    },
    middleware: (getDefaultMiddleware) => getDefaultMiddleware().concat(templateApi.middleware),
});

setupListeners(store.dispatch);

export type RootState = ReturnType<typeof store.getState>;
export type AppDispatch = typeof store.dispatch;

export default store;