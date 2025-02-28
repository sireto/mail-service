# Stage 1: set up the base image
ARG NODE_VERSION=18-alpine
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
ARG NEXT_PUBLIC_BASE_URL
ENV NEXT_PUBLIC_BASE_URL=${NEXT_PUBLIC_BASE_URL}
RUN yarn build

# Stage 2: Use lightweight node.js to run the app
ARG NODE_VERSION=18-alpine
FROM node:${NODE_VERSION}


# Remove default NGINX static files
# RUN rm -rf ./*

# Copy the frontend's public directory and the .next directory...
COPY --from=builder /app/public ./public
COPY --from=builder /app/.next/standalone ./
COPY --from=builder /app/.next/static ./.next/static

# # Copy the custom NGINX config
# COPY /config/nginx/nginx.conf /etc/nginx/nginx.conf


# Set the environment variables for production
ARG ENVIRONMENT=production
ENV NODE_ENV=${ENVIRONMENT}

EXPOSE 3000

# Start NGINX
# CMD ["nginx", "-g", "daemon off;"]

# Run the Next.js server
CMD ["node", "server.js"]