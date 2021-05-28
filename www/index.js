import {StampRender} from './stamp-maker.js';
import * as img2obj from 'img2obj'

function initStampRender(document) {
  const preview = document.getElementById("preview");

  const stampView = new StampRender(preview)

  let opt = img2obj.Options.new()

  let image = undefined
  let filename = undefined
  let objString = undefined

  const picker = document.getElementById("filepicker")
  picker.addEventListener('change', (evt) => {

    const files = evt.target.files
    if (files.length === 0) {
      return
    } else if (files.length !== 1) {
      throw Error("unexpected number of files " + files.length)
    }
    const nextFile = files[0]
    filename = nextFile.name

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
  optMaxEdge.value = opt.max_edge_mm.toString()
  optMaxEdge.addEventListener('change', (evt) => {
    const value = parseFloat(evt.target.value)
    opt.max_edge_mm = value
    reRender()
  })

  const optSmooth = document.getElementById("opt_smooth")
  optSmooth.value = opt.smooth_radius_mm.toString()
  optSmooth.addEventListener('change', (evt) => {
    const value = parseFloat(evt.target.value)
    opt.smooth_radius_mm = value
    reRender()
  })

  const optHeight = document.getElementById("opt_height")
  optHeight.value = opt.height_mm.toString()
  optHeight.addEventListener('change', (evt) => {
    const value = parseFloat(evt.target.value)
    opt.height_mm = value
    reRender()
  })

  const download = document.getElementById("start_download")
  download.addEventListener('click', (evt) => {
    const blob = new Blob([objString]);

    const baseName = filename.replace(/\.\w+$/, '')

    const link = document.createElement('a')
    if (link.download === undefined) {
      throw Error("download attr not defined")
    }
    const url = URL.createObjectURL(blob)
    link.setAttribute('href', url)
    link.setAttribute('download', `${baseName}.obj`)
    link.style.visibility = 'hidden'
    document.body.appendChild(link)
    link.click()
    document.body.removeChild(link)
  })

  function reRender() {
    if (!image) {
      return
    }

    download.disabled = false

    objString = img2obj.generate_from_bytes(image, opt)
    stampView.load(objString)
  }
}

img2obj.default().then(_ => {
  initStampRender(document)
})

