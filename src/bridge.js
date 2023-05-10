const { invoke } = window.__TAURI__.tauri;

// TODO: implementar para los POST con body
(function (open) {
    XMLHttpRequest.prototype.open = function(method, url, async, user, pass) {
        console.log('invoking ', method, url.slice(1));
        invoke(url.slice(1)).then(response => 
            Object.defineProperty(this, 'response', {
                get: () => response
            })
        );
        open.call(this, method, url, async, user, pass);
    }
})(XMLHttpRequest.prototype.open);

