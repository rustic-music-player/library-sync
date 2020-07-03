FROM node:alpine

WORKDIR /src

COPY ["package.json", "yarn.lock", "/src/"]

RUN yarn

COPY [".", "/src"]

RUN yarn build

ENTRYPOINT ["node", "lib/index.js"]
