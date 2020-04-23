FROM rust:1.42

EXPOSE 8000

RUN apt update

RUN apt install nodejs -y

RUN apt install npm -y

WORKDIR /app

COPY . .

RUN cargo install wasm-pack

RUN npm install

RUN npm run build

CMD npm run start

