import {StampRender} from './stamp-maker.js';
import * as img2obj from 'img2obj'

function initStampRender(document) {
  const preview = document.getElementById("preview");

  const stampView = new StampRender(preview)

  let opt = img2obj.Options.new()
  opt.invert = true
  opt.smooth = 10;

  let image = undefined;

  const picker = document.getElementById("filepicker")
  picker.addEventListener('change', (evt) => {

    const files = evt.target.files
    if (files.length !== 1) {
      throw Error("unexpected number of files " + files.length)
    }
    const nextFile = files[0]
    const reader = new FileReader()
    reader.onloadend = (e) => {
      image = new Uint8Array(reader.result)
      reRender()
    }
    reader.readAsArrayBuffer(nextFile)

  }, false)


  const optInvert = document.getElementById("opt_invert")
  optInvert.checked = opt.invert
  optInvert.addEventListener('change', (evt) => {
    opt.invert = evt.target.checked
    reRender()
  })

  const optMaxEdge = document.getElementById("opt_maxedge")
  optMaxEdge.value = opt.max_edge.toString()
  optMaxEdge.addEventListener('change', (evt) => {
    const value = parseInt(evt.target.value, 10)
    opt.max_edge = value
    reRender()
  })

  const optSmooth = document.getElementById("opt_smooth")
  optSmooth.value = opt.smooth.toString()
  optSmooth.addEventListener('change', (evt) => {
    const value = parseInt(evt.target.value, 10)
    opt.smooth = value
    reRender()
  })

  function reRender() {
    if (!image) {
      return
    }
    console.log(opt)
    const obj = img2obj.generate_from_bytes(image, opt)
    console.log(opt)
    stampView.load(obj)
  }
}

img2obj.default().then(_ => {
  initStampRender(document)
})

