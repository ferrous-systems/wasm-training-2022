document.querySelector('input[type=file]').onchange = (evt) => {
  postImage();
};
document.querySelector('select').onchange = (evt) => {
  postImage();
};

async function postImage() {
  let files = document.getElementById('files').files;
  if (!files.length) {
    return;
  }
  let file = files[0];
  let span = document.querySelector('span');
  span.innerText = "working...";

  let imgEl = document.querySelector('img');
  imgEl.src = URL.createObjectURL(file);
  imgEl.width = "500";

  let filter = document.querySelector("select").value.toLowerCase();
  if (filter == "none") {
    span.innerText = "done.";
    return;
  }

  let img = await file.arrayBuffer();
  let url = `/image?filter=${filter}`;
  let response = await fetch(url, {
    method: "POST",
    body: img,
  });

  if (!response.ok) {
    throw new Error(`HTTP error! Status: ${response.status}`);
  }

  let blob = await response.blob();
  imgEl.src = URL.createObjectURL(blob);
  span.innerText = "done.";
}
