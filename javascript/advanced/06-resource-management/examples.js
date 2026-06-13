// Run with: node examples.js

// --- Symbol.dispose + manual try/finally (what `using` desugars to) ---
console.log("=== withResource (manual using) ===");
{
  function openResource(name) {
    console.log(`  open ${name}`);
    return {
      name,
      [Symbol.dispose]() {
        console.log(`  close ${name}`);
      },
    };
  }

  function withResource(acquire, use) {
    const resource = acquire();
    try {
      return use(resource);
    } finally {
      resource[Symbol.dispose]();
    }
  }

  withResource(
    () => openResource("file.txt"),
    (file) => console.log(`  using ${file.name}`),
  );

  console.log("-- with an error: dispose still runs --");
  try {
    withResource(
      () => openResource("file2.txt"),
      () => {
        throw new Error("processing failed");
      },
    );
  } catch (err) {
    console.log(`  caught: ${err.message}`);
  }
}

// --- Symbol.asyncDispose ---
console.log("\n=== withAsyncResource (async cleanup) ===");
{
  function openConnection(name) {
    console.log(`  connect ${name}`);
    return {
      name,
      async [Symbol.asyncDispose]() {
        console.log(`  disconnect ${name}`);
      },
    };
  }

  async function withAsyncResource(acquireAsync, use) {
    const resource = await acquireAsync();
    try {
      return await use(resource);
    } finally {
      await resource[Symbol.asyncDispose]();
    }
  }

  await withAsyncResource(
    async () => openConnection("db"),
    async (conn) => console.log(`  querying ${conn.name}`),
  );
}

// --- Manual DisposableStack: LIFO cleanup of multiple resources ---
console.log("\n=== Manual DisposableStack (LIFO order) ===");
{
  function createDisposableStack() {
    const resources = [];
    return {
      use(resource) {
        resources.push(resource);
        return resource;
      },
      dispose() {
        for (let i = resources.length - 1; i >= 0; i--) {
          resources[i][Symbol.dispose]();
        }
        resources.length = 0;
      },
    };
  }

  const make = (name) => ({
    [Symbol.dispose]() {
      console.log(`  dispose ${name}`);
    },
  });

  const stack = createDisposableStack();
  stack.use(make("connection"));
  stack.use(make("transaction"));
  console.log("  (acquired connection, then transaction)");
  stack.dispose(); // transaction first, then connection
}

// --- Lazy resource: only dispose if actually created ---
console.log("\n=== Lazy resource ===");
{
  let created = false;
  function createLazyResource(factory) {
    let resource = null;
    return {
      get() {
        if (!created) {
          resource = factory();
          created = true;
        }
        return resource;
      },
      [Symbol.dispose]() {
        if (created) resource[Symbol.dispose]();
      },
    };
  }

  const lazy = createLazyResource(() => {
    console.log("  (factory called)");
    return { [Symbol.dispose]: () => console.log("  (disposed)") };
  });
  console.log("never used, then disposed:");
  lazy[Symbol.dispose](); // no-op -- factory was never called
}
