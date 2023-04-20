// setup double buffering
const canvas0 = document.querySelector("#canvas0");
const canvasContext0 = canvas0.getContext("2d");
const canvas1 = document.querySelector("#canvas1");
const canvasContext1 = canvas1.getContext("2d");

buffers = [
  {
    canvas: canvas0,
    canvasContext: canvasContext0,
  },
  {
    canvas: canvas1,
    canvasContext: canvasContext1,
  },
];

let visible_buf = 0;
buffers[visible_buf].canvas.style.visibility = "visible";
buffers[1 - visible_buf].canvas.style.visibility = "hidden";

// setup workers
const workers = [];
for (let i = 0; i < 4; i++) {
  workers[i] = new Worker("worker.js");
}

let animating = true; // animation running or not, changed via play/pause button
let renderingInput = false; // if scene is actively rerendered due to input

let pastTimestamp = null; // only non-null when animation is running
let animationTime = 0;

async function render(timestamp) {
  let start = new Date().getTime();

  if (animating) {
    if (pastTimestamp !== null) {
      deltaT = animating ? timestamp - pastTimestamp : 0;
      animationTime += deltaT;
    }
    pastTimestamp = timestamp;
  }

  // read input from DOM
  let resolution_factor = Math.pow(
    2,
    document.querySelector("#resolution").value
  );
  let fov = (document.querySelector("#fov").value / 180) * Math.PI;
  let supersampling = document.querySelector("#supersampling").checked;
  let camera_x = document.querySelector("#camera-x").value;
  let camera_y = document.querySelector("#camera-y").value;
  let camera_z = document.querySelector("#camera-z").value;
  let rotate_x = (document.querySelector("#rotate-x").value / 180) * Math.PI;
  let rotate_y = (document.querySelector("#rotate-y").value / 180) * Math.PI;
  let rotate_z = (document.querySelector("#rotate-z").value / 180) * Math.PI;

  let width = Math.min(
    Math.ceil(resolution_factor * document.body.scrollWidth),
    2 * 1920
  );
  let height = Math.min(
    Math.ceil(resolution_factor * document.body.scrollHeight),
    2 * 1080
  );

  buffers[1 - visible_buf].canvasContext.clearRect(0, 0, width, height);
  buffers[1 - visible_buf].canvas.width = width;
  buffers[1 - visible_buf].canvas.height = height;

  // parts of image that are rendered concurrently
  let splits = [
    {
      start_x: 0,
      end_x: Math.floor(width / 2),
      start_y: 0,
      end_y: Math.floor(height / 2),
    },
    {
      start_x: 0,
      end_x: Math.floor(width / 2),
      start_y: Math.floor(height / 2),
      end_y: height,
    },
    {
      start_x: Math.floor(width / 2),
      end_x: width,
      start_y: 0,
      end_y: Math.floor(height / 2),
    },
    {
      start_x: Math.floor(width / 2),
      end_x: width,
      start_y: Math.floor(height / 2),
      end_y: height,
    },
  ];

  // render scene in workers
  let finished = 0;
  for (const [i, { start_x, end_x, start_y, end_y }] of splits.entries()) {
    const worker = workers[i];
    worker.postMessage({
      width,
      height,
      start_x,
      end_x,
      start_y,
      end_y,
      fov,
      supersampling,
      camera_x,
      camera_y,
      camera_z,
      rotate_x,
      rotate_y,
      rotate_z,
      t: animationTime / 1000,
    });
    data = [];
    worker.onmessage = (msg) => {
      const { imageDataArray } = msg.data;
      const canvasImageData = buffers[
        1 - visible_buf
      ].canvasContext.createImageData(width, height);
      canvasImageData.data.set(imageDataArray);
      buffers[1 - visible_buf].canvasContext.putImageData(
        canvasImageData,
        0,
        0,
        start_x,
        start_y,
        end_x - start_x,
        end_y - start_y
      );

      finished++;
      if (finished === splits.length) {
        visible_buf = 1 - visible_buf;
        buffers[visible_buf].canvas.style.visibility = "visible";
        buffers[1 - visible_buf].canvas.style.visibility = "hidden";

        let end = new Date().getTime();
        document.querySelector("#frame-time").innerHTML = end - start;
        if (animating || renderingInput) requestAnimationFrame(render);
      }
    };
  }
}

requestAnimationFrame(render); // initial render

const startInput = () => {
  renderingInput = true;
  if (!animating) requestAnimationFrame(render);
};

const endInput = () => {
  renderingInput = false;
};

function playPause() {
  const playPauseBtn = document.querySelector(".play-pause-btn");
  playPauseBtn.classList.toggle("playing");

  if (playPauseBtn.classList.contains("playing")) {
    playPauseBtn.innerText = "Pause";
    animating = true;
    if (!renderingInput) requestAnimationFrame(render);
  } else {
    playPauseBtn.innerText = "Play";
    animating = false;
    pastTimestamp = null;
  }
}

document.querySelector("#resolution").addEventListener("mousedown", startInput);
document.querySelector("#fov").addEventListener("mousedown", startInput);
document
  .querySelector("#supersampling")
  .addEventListener("mousedown", startInput);
document.querySelector("#camera-x").addEventListener("mousedown", startInput);
document.querySelector("#camera-y").addEventListener("mousedown", startInput);
document.querySelector("#camera-z").addEventListener("mousedown", startInput);
document.querySelector("#rotate-x").addEventListener("mousedown", startInput);
document.querySelector("#rotate-y").addEventListener("mousedown", startInput);
document.querySelector("#rotate-z").addEventListener("mousedown", startInput);

document.querySelector("#resolution").addEventListener("mouseup", endInput);
document.querySelector("#fov").addEventListener("mouseup", endInput);
document.querySelector("#supersampling").addEventListener("mouseup", endInput);
document.querySelector("#camera-x").addEventListener("mouseup", endInput);
document.querySelector("#camera-y").addEventListener("mouseup", endInput);
document.querySelector("#camera-z").addEventListener("mouseup", endInput);
document.querySelector("#rotate-x").addEventListener("mouseup", endInput);
document.querySelector("#rotate-y").addEventListener("mouseup", endInput);
document.querySelector("#rotate-z").addEventListener("mouseup", endInput);
