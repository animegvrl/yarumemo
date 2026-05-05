import { Database } from "bun:sqlite";
import index from "./index.html";

const db = new Database("todo.sqlite", { create: true });
db.run("PRAGMA journal_mode = WAL;");
db.run(`
    CREATE TABLE IF NOT EXISTS tasks (
        id       INTEGER PRIMARY KEY,
        done     INTEGER NOT NULL DEFAULT 0 CHECK (done IN (0,1)),
        task     TEXT    NOT NULL,
        priority INTEGER NOT NULL DEFAULT 0
    );
`);

const server = Bun.serve({
    port: 3001,
    routes: {
        "/": index,
        "/list": () =>
        {
            using query = db.query("SELECT * FROM tasks ORDER BY priority DESC;");
            return new Response(JSON.stringify(query.all()));
        },
        "/task": {
            POST: async (req) => {
                const body = await req.json();

                using query = db.query("INSERT INTO tasks(task) VALUES ($task) RETURNING *;");
                const createdEntry = query.values({ $task: body.task })[0];

                return new Response(JSON.stringify(createdEntry));
            },
        },
        "/task/:id/task": {
            PUT: async (req) => {
                const body = await req.json();

                using query = db.query("UPDATE tasks SET task = $task WHERE id = $id;");
                query.values({ $id: req.params.id, $task: body.task });

                return new Response("OK");
            }
        },
        "/task/:id/priority/:priority": {
            POST: async (req) => {
                using query = db.query("UPDATE tasks SET priority = $priority WHERE id = $id;");
                query.values({ $id: req.params.id, $priority: req.params.priority });

                return new Response("OK");
            }
        },
        "/task/:id/check": {
            POST: async (req) => {
                using query = db.query("UPDATE tasks SET done = 1 WHERE id = $id;");
                query.values({ $id: req.params.id });

                return new Response("OK");
            }
        },
        "/task/:id/uncheck": {
            POST: async (req) => {
                using query = db.query("UPDATE tasks SET done = 0 WHERE id = $id;");
                query.values({ $id: req.params.id });

                return new Response("OK");
            }
        },
        "/task/:id/delete": {
            POST: async (req) => {
                using query = db.query("DELETE FROM tasks WHERE id = $id;");
                query.values({ $id: req.params.id });

                return new Response("OK");
            }
        },
    },

    fetch(req) {
        return new Response("Not Found", { status: 404 });
    },
});

console.log(`Server running at ${server.url}`);

for(const signal of ["beforeExit", "exit", "SIGINT", "SIGTERM", "SIGQUIT"])
{
    process.on(signal, async () => {
        console.log(`${signal}, stopping server.`);
        await server.stop();
        await db.close();
        console.log("bye.");
    });
}
