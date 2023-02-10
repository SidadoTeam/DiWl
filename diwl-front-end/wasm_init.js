
setTimeout(() => {
    const imports = getImports();
    initMemory(imports);
    load(wasm_buff, imports).then(({ instance, module }) => {
        finalizeInit(instance, module);
    });
}, 500);
