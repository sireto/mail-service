# Frontend — mail-service webapp

Next.js 16 App Router UI for the mail-service API. This file replaced the untouched
create-next-app boilerplate, which described nothing about this project.

## Requirements

Node >= 22 and Yarn 4 (via Corepack). `next` declares `node >=20.9.0` and Storybook's Vite
7 declares `^20.19.0 || >=22.12.0`; 22 satisfies both and is what CI and the Dockerfile use.

```bash
corepack enable
yarn install --immutable
```

## Environment

Both variables are inlined into the client bundle at build time, so they must be set when
`yarn build` runs — passing them only at runtime leaves them undefined in the browser.

- `NEXT_PUBLIC_BASE_URL` — API origin including the `/api` prefix, e.g. `http://localhost:8000/api`
- `NEXT_PUBLIC_NAMESPACE_ID` — namespace UUID the UI operates in

In Docker both arrive as build `args` (see frontend/Dockerfile and the root
docker-compose.yml).

## Commands

```bash
yarn dev              # dev server on :3000
yarn build            # production build (also runs TypeScript)
yarn lint             # eslint
yarn format           # prettier --write
yarn storybook        # component explorer on :6006
yarn build-storybook  # static Storybook, deployed to GitHub Pages from develop
```

There are no automated frontend tests. Storybook holds stories, not assertions.

## Layout

- `app/` — App Router routes. `app/services/` holds the RTK Query API slices, one per backend resource.
- `components/ui/` — shadcn/ui primitives. `components/common/` — project-specific shared components.
- `lib/type.ts` and `lib/type/` — Zod schemas and the types inferred from them; these are the contract with the API.
- `hooks/` — shared hooks. `useServerForm.ts` carries the credential-masking rules; read the comments there before changing it.
