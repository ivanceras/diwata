//
// this function intercepts XMLHttpRequest calls from elm
// and then we can then use it to redirect calls to neon js
// Use only in distributed desktop app
//
//
(function(XHR) {
    "use strict";
    console.log("intercepting..");
    
    var stats = [];
    var timeoutId = null;
    
    var open = XHR.prototype.open;
    var send = XHR.prototype.send;
    
    XHR.prototype.open = function(method, url, async, user, pass) {
        this._url = url;
        console.log("calling the url:", url);
        console.log("method: ", method);
        open.call(this, method, url, async, user, pass);
    };
    
    XHR.prototype.send = function(data) {
        console.log("data is trying to be sent..", data);
        var self = this;
        var start;
        var oldOnReadyStateChange;
        var url = this._url;
        
        function onReadyStateChange() {
            if(self.readyState == 4 /* complete */) {
                console.log("status: ", self.status);
                console.log("got response: ", self.response);
                console.log("got responseText: ", self.responseText);

                // we need to store the original response before any modifications
                // because the next step will erase everything it had
                var original_response = self.response;
                var original_responseText = self.responseText;

                // here we "kill" the response property of this request
                // and we set it to writable
                Object.defineProperty(self, "response", {writable: true});

                var modified_response = self.response 
                // pass as is
                self.response = original_response;

                var time = new Date() - start;                
                stats.push({
                    url: url,
                    duration: time                    
                });
                
                if(!timeoutId) {
                    timeoutId = window.setTimeout(function() {
                        var xhr = new XHR();
                        xhr.noIntercept = true;
                        xhr.open("POST", "/clientAjaxStats", true);
                        xhr.setRequestHeader("Content-type","application/json");
                        xhr.send(JSON.stringify({ stats: stats } ));                        
                        
                        timeoutId = null;
                        stats = []; 
                    }, 2000);
                }                
            }
            
            if(oldOnReadyStateChange) {
                oldOnReadyStateChange();
            }
        }
        
        if(!this.noIntercept) {
            start = new Date();
            
            if(this.addEventListener) {
                this.addEventListener("readystatechange", onReadyStateChange, false);
            } else {
                oldOnReadyStateChange = this.onreadystatechange; 
                this.onreadystatechange = onReadyStateChange;
            }
        }
        
        send.call(this, data);
    }
})(XMLHttpRequest);
