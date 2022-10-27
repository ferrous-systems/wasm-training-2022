importScripts("web_image_filter.js");

let apply_filter;

function log(line) {
  self.postMessage({type: 'log', line: line});
}

self.onmessage = async (event) => {
  // The first message should be that startup message, carrying the URL
  if (event.data.type == 'startup') {
    log("starting...");
    apply_filter = wasm_bindgen.apply_filter;
    await wasm_bindgen('./web_image_filter_bg.wasm');
    log("startup done");
    return;
  }

  if (event.data.type == 'image') {
    log('starting image handling');
    let img = event.data.image;
    let filter = event.data.filter || 'valencia';
    let result = apply_filter(new Uint8Array(img), filter);
    self.postMessage({type: 'image', buffer: result});
    log('image done');
  }
}
