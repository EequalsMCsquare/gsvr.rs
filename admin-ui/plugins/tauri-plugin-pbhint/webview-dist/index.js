var n=function(){return n=Object.assign||function(n){for(var t,e=1,r=arguments.length;e<r;e++)for(var o in t=arguments[e])Object.prototype.hasOwnProperty.call(t,o)&&(n[o]=t[o]);return n},n.apply(this,arguments)};function t(n,t){void 0===t&&(t=!1);var e=window.crypto.getRandomValues(new Uint32Array(1))[0],r="_".concat(e);return Object.defineProperty(window,r,{value:function(e){return t&&Reflect.deleteProperty(window,r),null==n?void 0:n(e)},writable:!1,configurable:!0}),e}function e(e,r){return void 0===r&&(r={}),function(n,t,e,r){return new(e||(e=Promise))((function(o,i){function c(n){try{u(r.next(n))}catch(n){i(n)}}function a(n){try{u(r.throw(n))}catch(n){i(n)}}function u(n){var t;n.done?o(n.value):(t=n.value,t instanceof e?t:new e((function(n){n(t)}))).then(c,a)}u((r=r.apply(n,t||[])).next())}))}(this,void 0,void 0,(function(){return function(n,t){var e,r,o,i,c={label:0,sent:function(){if(1&o[0])throw o[1];return o[1]},trys:[],ops:[]};return i={next:a(0),throw:a(1),return:a(2)},"function"==typeof Symbol&&(i[Symbol.iterator]=function(){return this}),i;function a(i){return function(a){return function(i){if(e)throw new TypeError("Generator is already executing.");for(;c;)try{if(e=1,r&&(o=2&i[0]?r.return:i[0]?r.throw||((o=r.return)&&o.call(r),0):r.next)&&!(o=o.call(r,i[1])).done)return o;switch(r=0,o&&(i=[2&i[0],o.value]),i[0]){case 0:case 1:o=i;break;case 4:return c.label++,{value:i[1],done:!1};case 5:c.label++,r=i[1],i=[0];continue;case 7:i=c.ops.pop(),c.trys.pop();continue;default:if(!((o=(o=c.trys).length>0&&o[o.length-1])||6!==i[0]&&2!==i[0])){c=0;continue}if(3===i[0]&&(!o||i[1]>o[0]&&i[1]<o[3])){c.label=i[1];break}if(6===i[0]&&c.label<o[1]){c.label=o[1],o=i;break}if(o&&c.label<o[2]){c.label=o[2],c.ops.push(i);break}o[2]&&c.ops.pop(),c.trys.pop();continue}i=t.call(n,c)}catch(n){i=[6,n],r=0}finally{e=o=0}if(5&i[0])throw i[1];return{value:i[0]?i[1]:void 0,done:!0}}([i,a])}}}(this,(function(o){return[2,new Promise((function(o,i){var c=t((function(n){o(n),Reflect.deleteProperty(window,"_".concat(a))}),!0),a=t((function(n){i(n),Reflect.deleteProperty(window,"_".concat(c))}),!0);window.__TAURI_IPC__(n({cmd:e,callback:c,error:a},r))}))]}))}))}Object.freeze({__proto__:null,transformCallback:t,invoke:e,convertFileSrc:function(n,t){void 0===t&&(t="asset");var e=encodeURIComponent(n);return navigator.userAgent.includes("Windows")?"https://".concat(t,".localhost/").concat(e):"".concat(t,"://").concat(e)}});var r={options:function(){return e("plugin:pbhint|get_options")},hint:function(n){return e("plugin:pbhint|try_hint",{key:n})}};export{r as default};
