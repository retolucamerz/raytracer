importScripts("./pkg/web.js");

let rustWasm = null;

self.onmessage = async (msg) => {
  if (rustWasm === null) {
    rustWasm = await wasm_bindgen("./pkg/web_bg.wasm");
  }

  const {
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
    t,
  } = msg.data;

  rustWasm.generate_image(
    width,
    height,
    start_x,
    end_x,
    start_y,
    end_y,
    camera_x,
    camera_y,
    camera_z,
    rotate_x,
    rotate_y,
    rotate_z,
    fov,
    supersampling,
    t
  );
  const wasmByteMemoryArray = new Uint8Array(rustWasm.memory.buffer);
  const outputPointer = rustWasm.get_output_buffer_pointer();
  const imageDataArray = wasmByteMemoryArray.slice(
    outputPointer,
    outputPointer + width * height * 4
  );
  self.postMessage({ imageDataArray });
};
