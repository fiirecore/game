register_plugin = function (importObject) {

    const name = "Cookie saver";
    const version = "1.0";

    importObject.env.getCookie = function getCookie(cname) {
        var properName = consume_js_object(cname);
        console.info("Getting cookie: \"" + properName + "\"");
        var nameEquals = properName + "=";
        var decodedCookie = decodeURIComponent(document.cookie);
        var ca = decodedCookie.split(';');
        for (var i = 0; i < ca.length; i++) {
            var c = ca[i];
            while (c.charAt(0) == ' ') {
                c = c.substring(1);
            }
            if (c.indexOf(nameEquals) == 0) {
                var cookie = c.substring(nameEquals.length, c.length);
                return js_object(cookie);
            }
        }
        console.warn("Could not find cookie with name: \"" + properName + "\"");
        return js_object("Missing cookie");
    }
    
    importObject.env.setCookie = function setCookie(cname, cvalue) {
        var value = consume_js_object(cvalue);
        var properName = consume_js_object(cname);
        console.info('Saving cookie named \"' + properName + '\" with data: ' + value);
        var exdays = 365 * 100;
        var d = new Date();
        d.setTime(d.getTime() + (exdays*24*60*60*1000));
        var expires = 'expires='+ d.toUTCString();
        document.cookie = properName + '=' + value + ';' + expires + ';path=/';
        console.info("Saved cookie: \"" + properName + "\"");
    }

}

miniquad_add_plugin({ register_plugin });