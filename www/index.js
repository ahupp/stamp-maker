import {StampRender} from './stamp-maker.js';
import * as img2obj from 'img2obj'

function initStampRender(document) {
  const container = document.createElement('div');
  document.body.appendChild(container);

  const stampView = new StampRender(container)

  const picker = document.getElementById("filepicker")
  picker.addEventListener('change', (evt) => {

    const files = evt.target.files
    if (files.length !== 1) {
      throw Error("unexpected number of files " + files.length)
    }
    const nextFile = files[0]
    const reader = new FileReader()
    reader.onloadend = (e) => {
      const u8array = new Uint8Array(reader.result)
      const opt = img2obj.Options.new()
      opt.invert = true
      const obj = img2obj.generate_from_bytes(u8array, opt)
      stampView.load(obj)
    }
    reader.readAsArrayBuffer(nextFile)

  }, false)
}

img2obj.default().then(_ => {
  initStampRender(document)
})

