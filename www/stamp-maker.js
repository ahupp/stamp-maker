import * as THREE from 'three'
import { OrbitControls } from 'three/examples/jsm/controls/OrbitControls.js'
import {OBJLoader} from 'three/examples/jsm/loaders/OBJLoader.js'

export class StampRender {
  constructor(container) {

    const camera = new THREE.PerspectiveCamera(45,
      container.clientWidth / container.clientHeight, 1, 1000)
    camera.position.z = 3

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
    console.log(container.clientWidth, container.clientHeight)
    renderer.setSize(container.clientWidth, container.clientHeight)
    renderer.setClearColor(new THREE.Color("hsl(0, 0%, 10%)"))

    container.appendChild(renderer.domElement)

    const controls = new OrbitControls(camera, renderer.domElement)
    controls.enableDamping = true
    controls.dampingFactor = 0.25

    window.addEventListener( 'resize', onWindowResize, false);

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
    const object = loader.parse(model)
    if (this.object)  {
      this.scene.remove(this.object)
    }
    this.object = object
    this.scene.add(this.object)
  }
}

