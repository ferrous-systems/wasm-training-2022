function typedArrayToURL(typedArray, mimeType) {
  return URL.createObjectURL(
    new Blob([typedArray.buffer], { type: mimeType })
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
    worker.postMessage({type: 'image', image: img, filter});
  };

  reader.readAsArrayBuffer(file);
}

document.querySelector('input[type=file]').onchange = (evt) => {
  imageFilter();
};
document.querySelector('select').onchange = (evt) => {
  imageFilter();
};

const worker = new Worker("worker.js");
worker.postMessage({type: 'startup'});

worker.onmessage = (event) => {
  // {type: log, line: ...} messages are appended to a log textarea:
  if (event.data.type == 'log') {
    console.log(event.data.line);
    return;
  }

  if (event.data.type == 'image') {
    let result = event.data.buffer;
    let blobUrl = typedArrayToURL(result, "image/png");
    let el = document.querySelector('img');
    el.src = blobUrl;
    el.width = "500";
    var span = document.querySelector('span');
    span.innerText = "done.";
    return;
  }
}
