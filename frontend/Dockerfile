# This was node:18-alpine, which cannot build the app: next 16 declares engines
# node >=20.9.0 and vite 7 (Storybook) declares ^20.19.0 || >=22.12.0. Pinned to 22, the
# version the build was actually verified on.
ARG NODE_VERSION=22-alpine
FROM node:${NODE_VERSION} AS builder

WORKDIR /app
RUN apk add --no-cache libc6-compat

# Enable Corepack (for Yarn version management)
RUN corepack enable

# Install all the required dependencies...
COPY package.json yarn.lock ./
COPY .yarnrc.yml ./
RUN yarn install --immutable

# Build the Next.js app...
COPY . .

# NEXT_PUBLIC_* values are inlined into the client bundle at build time, so every one of
# them has to arrive as a build argument. NEXT_PUBLIC_NAMESPACE_ID was previously only ever
# passed as a runtime environment variable, which meant it was undefined in the browser and
# the app silently fell back to a hardcoded namespace UUID.
ARG NEXT_PUBLIC_BASE_URL
ARG NEXT_PUBLIC_NAMESPACE_ID
ENV NEXT_PUBLIC_BASE_URL=${NEXT_PUBLIC_BASE_URL}
ENV NEXT_PUBLIC_NAMESPACE_ID=${NEXT_PUBLIC_NAMESPACE_ID}
RUN yarn build

# Stage 2: Use lightweight node.js to run the app
ARG NODE_VERSION=22-alpine
FROM node:${NODE_VERSION}

# The previous runtime stage had no WORKDIR, so the standalone server was copied into / .
WORKDIR /app

COPY --from=builder --chown=node:node /app/public ./public
COPY --from=builder --chown=node:node /app/.next/standalone ./
COPY --from=builder --chown=node:node /app/.next/static ./.next/static

ARG ENVIRONMENT=production
ENV NODE_ENV=${ENVIRONMENT}
ENV PORT=3000
ENV HOSTNAME=0.0.0.0

# node:*-alpine ships an unprivileged `node` user; the previous image ran as root.
USER node

EXPOSE 3000

CMD ["node", "server.js"]
