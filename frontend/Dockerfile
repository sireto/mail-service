# Stage 1: set up the base image
FROM node:18-alpine AS base
WORKDIR /app
RUN apk add --no-cache libc6-compat

# Enable Corepack (for Yarn version management)
RUN corepack enable

# Stage 2: Install dependencies only when needed
FROM base AS deps
COPY package.json yarn.lock ./
COPY .yarnrc.yml ./

RUN yarn install --immutable

# Stage 3: Rebuild the source code only when needed
FROM base AS builder
WORKDIR /app
COPY --from=deps /app/node_modules ./node_modules
COPY . .
RUN yarn build

# Stage 4: Production image, copy all the files and run next
FROM base AS runner

# Copy the frontend's public directory and the .next directory...
COPY --from=builder /app/public ./public
COPY --from=builder /app/.next/standalone ./
COPY --from=builder /app/.next/static ./.next/static

# Set the environment variables for production
ENV NODE_ENV=production

EXPOSE 8021

# server.js is created by next build from the standalone output...
CMD ["node", "server.js"]
