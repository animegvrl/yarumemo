import { Database } from "bun:sqlite";

export const db = new Database("todo.sqlite", { create: true });
db.run("PRAGMA journal_mode = WAL;");
db.run(`
    CREATE TABLE IF NOT EXISTS tasks (
        id       INTEGER PRIMARY KEY,
        done     INTEGER NOT NULL DEFAULT 0 CHECK (done IN (0,1)),
        task     TEXT    NOT NULL,
        priority INTEGER NOT NULL DEFAULT 0
    );
`);

interface ITask
{
    id?: number;
    done?: number;
    task: string;
    priority?: number;
}

export function validateBody(obj: unknown)
{
    if(typeof (obj as any).task === 'string') return obj as ITask;
    else throw new Error("Missing mandatory field task.");
}

export const task =
{
    create(task: ITask)
    {
        using query = db.query("INSERT INTO tasks(task) VALUES ($task) RETURNING *;");
        return query.values({ $task: task.task })[0];
    },
    queryAll()
    {
        using query = db.query("SELECT * FROM tasks ORDER BY priority DESC;");
        return query.all();
    },
    updateTask(id: number, task: string)
    {
        using query = db.query("UPDATE tasks SET task = $task WHERE id = $id;");
        return query.values({ $id: id, $task: task });
    },
    updatePriority(id: number, priority: number)
    {
        using query = db.query("UPDATE tasks SET priority = $priority WHERE id = $id;");
        return query.values({ $id: id, $priority: priority });
    },
    updateDone(id: number, done: number)
    {
        using query = db.query("UPDATE tasks SET done = $done WHERE id = $id;");
        query.values({ $id: id, $done: done });
    },
    delete(id: number)
    {
        using query = db.query("DELETE FROM tasks WHERE id = $id;");
        query.values({ $id: id });
    }
};
