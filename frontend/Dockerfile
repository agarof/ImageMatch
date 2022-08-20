FROM node:18-alpine AS builder

WORKDIR /app
ARG VITE_BACKEND_HOST
ARG VITE_IMAGE_HOST

COPY package.json ./
COPY package-lock.json ./
RUN npm install
COPY . ./
RUN echo "VITE_BACKEND_HOST=${VITE_BACKEND_HOST}\nVITE_IMAGE_HOST=${VITE_IMAGE_HOST}" >> .env
RUN npm run build

FROM nginx:1.23-alpine
COPY --from=builder /app/build /usr/share/nginx/html
# RUN rm /etc/nginx/conf.d/default.conf
# COPY nginx.conf /etc/nginx/conf.d
EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
