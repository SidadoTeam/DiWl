// ==UserScript==
// @name         diwl
// @namespace    http://tampermonkey.net/
// @version      0.1
// @description  try to take over the world!
// @author       You
// @match        *://www.youtube.com/*
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
    var timestamp = new Date().getTime();

    console.log('hello tampermonkey',timestamp);
    addElement('script',"http://localhost:8000/bundle.js?v="+timestamp);
    addElement('script','http://localhost:8000/wasm_data.js?v='+timestamp);
    //addElement('script','http://localhost:8000/dist/trunk-template-a1ae4d3b612e8c1e.js');
    addElement('script','http://localhost:8000/trunk.js?v='+timestamp);
    addElement('script','http://localhost:8000/wasm_init.js?v='+timestamp);

    //addElement('script','https://cdn.tailwindcss.com');

    function addElement(e,url){
        var script = document.createElement(e);
        script.src = url;
        document.head.appendChild(script);
    }

    // await apiPromise;
    // let res = await getwCommon(1, 200);
    // console.log(res);
})();