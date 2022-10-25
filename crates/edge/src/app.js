function postImage() {
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
    var url = `/image?filter=${filter}`;
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
  postImage();
};
document.querySelector('select').onchange = (evt) => {
  postImage();
};
