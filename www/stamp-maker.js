import * as THREE from 'three'
import { OrbitControls } from 'three/examples/jsm/controls/OrbitControls.js'
import {OBJLoader} from 'three/examples/jsm/loaders/OBJLoader.js'

export class StampRender {
  constructor(container) {

    this.fov = 45

    const aspect = container.clientWidth / container.clientHeight
    const camera = new THREE.PerspectiveCamera(this.fov, aspect, 1, 1000)
    this.camera = camera

    const scene = new THREE.Scene()
    const ambient = new THREE.AmbientLight(0xffffff, .25)
    scene.add(ambient)

    const keyLight = new THREE.DirectionalLight(new THREE.Color('hsl(30, 100%, 75%)'), 1.0)
    keyLight.position.set(-100, 0, 100)

    const fillLight = new THREE.DirectionalLight(new THREE.Color('hsl(240, 100%, 75%)'), 0.75)
    fillLight.position.set(100, 0, 100)

    const backLight = new THREE.DirectionalLight(0xffffff, 1.0)
    backLight.position.set(100, 0, -100).normalize()

    scene.add(keyLight)
    scene.add(fillLight)
    scene.add(backLight)

    this.scene = scene

    const renderer = new THREE.WebGLRenderer()
    renderer.setPixelRatio(window.devicePixelRatio)
    console.log(container.clientHeight, container.clientWidth)
    renderer.setSize(container.clientWidth, container.clientHeight)
    renderer.setClearColor(new THREE.Color("hsl(0, 0%, 10%)"))

    container.appendChild(renderer.domElement)

    const controls = new OrbitControls(camera, renderer.domElement)
    controls.enableDamping = true
    controls.dampingFactor = 0.25

    window.addEventListener('resize', onWindowResize, false);

    function onWindowResize() {
        camera.aspect = container.clientWidth / container.clientHeight
        camera.updateProjectionMatrix();
        renderer.setSize(container.clientWidth, container.clientHeight)
    }

    function render() {
      requestAnimationFrame(render)
      controls.update()
      renderer.render(scene, camera)
    }
    render()
  }

  load(model) {
    const loader = new OBJLoader()
    const objGroup = loader.parse(model)
    const mesh = objGroup.children[0]

    // center in viewport
    mesh.geometry.computeBoundingBox()
    const box = mesh.geometry.boundingBox

    const offset = box.max.sub(box.min).divideScalar(2.0)
    mesh.position.x -= offset.x
    mesh.position.y -= offset.y
    mesh.geometry.computeBoundingBox()

    if (this.mesh)  {
      this.scene.remove(this.mesh)
    } else {
      // On first load, backup camera to fit whole model
      const opposite = 1.2*(mesh.geometry.boundingBox.max.y / 2)
      const adjacent = opposite/Math.tan((Math.PI/180.)*this.fov/2)
      this.camera.position.z = adjacent
      this.camera.lookAt(0, 0, 0)
    }
    this.mesh = mesh
    this.scene.add(this.mesh)
  }
}

