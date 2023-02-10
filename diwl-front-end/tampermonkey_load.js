// ==UserScript==
// @name         New Userscript
// @namespace    http://tampermonkey.net/
// @version      0.1
// @description  try to take over the world!
// @author       You
// @match        *
// @icon
// @grant GM_xmlhttpRequest
// @grant GM_getValue
// @grant GM_setClipboard
// @grant unsafeWindow
// @grant window.close
// @grant window.focus
// @grant window.onurlchange
// ==/UserScript==

(function() {
    'use strict';
    console.log('hello tampermonkey');

    //let url = "http://localhost:8080/bundle.js";
    //addElement('script',url);

    addElement('script','http://localhost:8000/wasm_data.js');
    //addElement('script','http://localhost:8000/dist/trunk-template-a1ae4d3b612e8c1e.js');
    addElement('script','http://localhost:8000/trunk.js');
    addElement('script','http://localhost:8000/wasm_init.js');

    function addElement(e,url){
        var script = document.createElement(e);
        script.src = url;
        document.head.appendChild(script);
    }
})();