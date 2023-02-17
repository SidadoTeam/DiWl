
setTimeout(() => {
    const imports = getImports();
    initMemory(imports);
    load(wasm_buff, imports).then(({ instance, module }) => {
        finalizeInit(instance, module);
    });
}, 500);

function openMpopup() {
    console.log("open popup");
    let d = document.getElementById("m_popup");
    d.show();
}

window.openMpopup = openMpopup;