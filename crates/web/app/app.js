import init, { apply_filter } from './image_filter.js';

await init();

document.querySelector('input[type=file]').onchange = (evt) => {
  imageFilter();
};
document.querySelector('select').onchange = (evt) => {
  imageFilter();
};

function typedArrayToURL(typedArray) {
  return URL.createObjectURL(
    new Blob([typedArray.buffer], { type: "image/png" })
  );
}

function imageFilter() {
  var files = document.getElementById('files').files;
  if (!files.length) {
    return;
  }
  var file = files[0];
  var span = document.querySelector('span');
  span.innerText = "working...";

  var el = document.querySelector('img');
  el.src = URL.createObjectURL(file);
  el.width = "500";

  var filter = document.querySelector("select").value.toLowerCase();
  if (filter == "none") {
    span.innerText = "done.";
    return;
  }

  var reader = new FileReader();
  reader.onload = function(readerEvt) {
    var img = readerEvt.target.result;
    let result = apply_filter(new Uint8Array(img), filter);
    let blobUrl = typedArrayToURL(result);
    let el = document.querySelector('img');
    el.src = blobUrl;
    el.width = "500";
    var span = document.querySelector('span');
    span.innerText = "done.";
  };

  reader.readAsArrayBuffer(file);
}
