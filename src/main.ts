import { validateBody, task, db } from "./db";
import index from "./index.html";

const server = Bun.serve({
    port: 3001,
    routes: {
        "/": index,
        "/list": () => new Response(JSON.stringify(task.queryAll())),
        "/task": {
            POST: async (req) => {
                const body = validateBody(await req.json());

                return new Response(JSON.stringify(task.create(body)));
            },
        },
        "/task/:id/task": {
            PUT: async (req) => {
                const body = validateBody(await req.json());

                task.updateTask(parseInt(req.params.id), body.task);

                return new Response("OK");
            }
        },
        "/task/:id/priority/:priority": {
            POST: async (req) => {
                task.updatePriority(parseInt(req.params.id), parseInt(req.params.priority));

                return new Response("OK");
            }
        },
        "/task/:id/check": {
            POST: async (req) => {
                task.updateDone(parseInt(req.params.id), 1);

                return new Response("OK");
            }
        },
        "/task/:id/uncheck": {
            POST: async (req) => {
                task.updateDone(parseInt(req.params.id), 0);

                return new Response("OK");
            }
        },
        "/task/:id/delete": {
            POST: async (req) => {
                task.delete(parseInt(req.params.id));

                return new Response("OK");
            }
        },
    },

    fetch(_req) {
        return new Response("Not Found", { status: 404 });
    },
});

console.log(`Server running at ${server.url}`);

for(const signal of ["beforeExit", "exit", "SIGINT", "SIGTERM", "SIGQUIT"])
{
    process.on(signal, async () => {
        console.log(`${signal}, stopping server.`);
        await server.stop();
        db.close();
        console.log("bye.");
    });
}
