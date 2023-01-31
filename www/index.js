import { StampRender } from "./stamp-render.js";
// Workaround for vite not supporting "files" in package.json
import wasmUrl from 'stamp-maker-wasm/index_bg.wasm?url'
import initStampMaker, * as stamp_maker from "stamp-maker-wasm";

export async function initStampRender(document) {
  await initStampMaker(wasmUrl);

  const preview = document.getElementById("preview");

  const stampView = new StampRender(preview);

  let opt = stamp_maker.Options.new();

  let image = undefined;
  let filename = undefined;
  let objString = undefined;

  const picker = document.getElementById("filepicker");
  picker.addEventListener(
    "change",
    (evt) => {
      const files = evt.target.files;
      if (files.length === 0) {
        return;
      } else if (files.length !== 1) {
        throw Error("unexpected number of files " + files.length);
      }
      const nextFile = files[0];
      filename = nextFile.name;

      const reader = new FileReader();
      reader.onloadend = (e) => {
        image = new Uint8Array(reader.result);
        reRender();
      };
      reader.readAsArrayBuffer(nextFile);
    },
    false
  );

  const optInvert = document.getElementById("opt_invert");
  optInvert.checked = opt.invert;
  optInvert.addEventListener("change", (evt) => {
    opt.invert = evt.target.checked;
    reRender();
  });

  const optMaxEdge = document.getElementById("opt_maxedge");
  optMaxEdge.value = opt.max_edge_mm.toString();
  optMaxEdge.addEventListener("change", (evt) => {
    const value = parseFloat(evt.target.value);
    opt.max_edge_mm = value;
    reRender();
  });

  const optSmooth = document.getElementById("opt_smooth");
  optSmooth.value = opt.smooth_radius_mm.toString();
  optSmooth.addEventListener("change", (evt) => {
    const value = parseFloat(evt.target.value);
    opt.smooth_radius_mm = value;
    reRender();
  });

  const optHeight = document.getElementById("opt_height");
  optHeight.value = opt.height_mm.toString();
  optHeight.addEventListener("change", (evt) => {
    const value = parseFloat(evt.target.value);
    opt.height_mm = value;
    reRender();
  });

  const download = document.getElementById("start_download");
  download.addEventListener("click", (evt) => {
    const blob = new Blob([objString]);

    const baseName = filename.replace(/\.\w+$/, "");

    const link = document.createElement("a");
    if (link.download === undefined) {
      throw Error("download attr not defined");
    }
    const url = URL.createObjectURL(blob);
    link.setAttribute("href", url);
    link.setAttribute("download", `${baseName}.obj`);
    link.style.visibility = "hidden";
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
  });

  function reRender() {
    if (!image) {
      return;
    }

    download.disabled = false;

    objString = stamp_maker.generate_from_bytes(image, opt);
    stampView.load(objString);
  }
}
