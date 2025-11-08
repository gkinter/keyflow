FROM node:20-alpine AS builder
WORKDIR /app

ENV NODE_ENV=development

COPY package*.json ./
RUN npm ci

COPY . .

RUN npm run build

FROM node:20-alpine AS runner
WORKDIR /app
ENV NODE_ENV=production

COPY package*.json ./
RUN npm ci --omit=dev

COPY --from=builder /app/build ./build

RUN addgroup -g 10001 app && adduser -D -G app -u 10001 app
USER app

EXPOSE 4173

CMD ["node", "build/index.js"]

