function typedArrayToURL(typedArray, mimeType) {
  return URL.createObjectURL(
    new Blob([typedArray.buffer], { type: mimeType })
  );
}

function bufferToURL(buffer, mimeType) {
  return URL.createObjectURL(
    new Blob([buffer], { type: mimeType })
  );
}

function readBlob() {
  var files = document.getElementById('files').files;
  if (!files.length) {
    return;
  }
  var file = files[0];
  var span = document.querySelector('span');
  span.innerText = "working...";

  var MIMEType = file.type;
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
    var url = `/image?filter=${filter}`;
    console.log(`posting to ${url}`);
    fetch(url, {
      method: "POST",
      body: img,
    })
    .then((response) => {
      if (!response.ok) {
        throw new Error(`HTTP error! Status: ${response.status}`);
      }

      return response.blob();
    })
    .then((response) => {
      el.src = URL.createObjectURL(response);
      var span = document.querySelector('span');
      span.innerText = "done.";
    });
  };

  reader.readAsArrayBuffer(file);
}

document.querySelector('input[type=file]').onchange = (evt) => {
  readBlob();
};
document.querySelector('select').onchange = (evt) => {
  readBlob();
};
