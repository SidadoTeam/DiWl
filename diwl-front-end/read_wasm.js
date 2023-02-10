
var fs = require('fs');

console.log("hello");

// var wasm_path = "dist/trunk-template-933b8676464ca46_bg.wasm";

fs.readdir("dist/", (_, files) => {
    files.forEach((file) => {
        if (file.endsWith(".wasm"))
            console.log(file);
        fs.readFile("dist/" + file, function (err, data) {
            //console.log();
            let wasm_data = data.toString("base64");
            let file_content = "var wasm_data='" + wasm_data + "';function base64ToArrayBuffer(base64) {var binary_string = atob(base64);var len = binary_string.length;var bytes = new Uint8Array(len);for (var i = 0; i < len; i++) {bytes[i] = binary_string.charCodeAt(i);}return bytes.buffer;}let wasm_buff = base64ToArrayBuffer(wasm_data);console.log(wasm_buff);";
            fs.writeFile('wasm_data.js', file_content, { flag: 'w' }, (err) => {
                if (err) {
                    console.error(err)
                }
            })
        })
    });
});




