# yarumemo

A simple keyboard-first todo list (web)app.

### ❗❗❗ very early "usable" development version, everything including keybinds is going to eventually change. back up your databases before updating.

## running yarumemo

yarumemo uses [bun](https://bun.sh/) to serve html, api and for database operations.

To run yarumemo you have to,

- Download and Install [bun](https://bun.sh/)
- Run `bun run dev` to launch a local server
- Access yarumemo at [http://localhost:3001/](http://localhost:3001/)
- (additionally if port `3001` is taken on your machine, you have to manually change it in `src/main.ts`)

## keymap

| key           | function                  |
|---------------|---------------------------|
| arrow up/down | navigation                |
| space         | toggle checkbox           |
| n             | new task                  |
| e             | edit task                 |
| p             | edit task priority        |
| d             | delete task (hard delete) |
