const workers = [];
for (let i = 0; i < 4; i++) {
  workers[i] = new Worker("worker.js");
}

function render() {
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

  const canvas = document.querySelector("canvas");
  const canvasContext = canvas.getContext("2d");
  const canvasImageData = canvasContext.createImageData(width, height);
  canvasContext.clearRect(0, 0, width, height);
  canvas.width = width;
  canvas.height = height;

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

  let start = new Date().getTime();
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
    });
    worker.onmessage = (msg) => {
      const { imageDataArray } = msg.data;
      canvasImageData.data.set(imageDataArray);
      canvasContext.putImageData(
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
        let end = new Date().getTime();
        document.querySelector("#frame-time").innerHTML = end - start;
      }
    };
  }
}

render();

document.querySelector("#resolution").addEventListener("change", render);
document.querySelector("#fov").addEventListener("change", render);
document.querySelector("#supersampling").addEventListener("change", render);
document.querySelector("#camera-x").addEventListener("change", render);
document.querySelector("#camera-y").addEventListener("change", render);
document.querySelector("#camera-z").addEventListener("change", render);
document.querySelector("#rotate-x").addEventListener("change", render);
document.querySelector("#rotate-y").addEventListener("change", render);
document.querySelector("#rotate-z").addEventListener("change", render);
