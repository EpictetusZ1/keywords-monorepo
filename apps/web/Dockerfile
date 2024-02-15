FROM node:18-alpine AS base
LABEL authors="EpictetusZ1"

# Set working directory to the root of your monorepo
WORKDIR /app

COPY package*.json ./
COPY apps/web/package*.json apps/web/

RUN npm install

# Copy your monorepo into the Docker image
COPY . .

# Set the working directory to the specific web app within your monorepo
WORKDIR /app/apps/web

# Continue with your project-specific setup, such as exposing ports and defining the runtime command
EXPOSE 3000

CMD ["npm", "start"]
