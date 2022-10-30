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

async function imageFilter() {
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
  let result = apply_filter(new Uint8Array(img), filter);
  let blobUrl = typedArrayToURL(result);
  imgEl.src = blobUrl;
  imgEl.width = "500";
  span.innerText = "done.";
}
