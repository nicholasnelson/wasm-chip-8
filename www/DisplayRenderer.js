import * as THREE from 'three';

// eslint-disable-next-line import/no-unresolved
import { memory } from 'chip8/chip_8_emu_bg';

export default class DisplayRenderer {
  constructor(displayPtr, container) {
    this.displayPtr = displayPtr;

    this.fov = 65.3;
    this.near = 0.1;
    this.far = 1000;
    this.pixelWidth = 64;
    this.pixelHeight = 32;

    // Create Renderer
    this.renderer = new THREE.WebGLRenderer();
    container.append(this.renderer.domElement);

    this.camera = new THREE.PerspectiveCamera(
      this.fov,
      this.width() / this.height(),
      this.near,
      this.far,
    );

    this.scene = new THREE.Scene();
    const emuDisplayBuffer = new Uint8Array(
      memory.buffer,
      displayPtr,
      this.pixelWidth * this.pixelHeight * 3,
    );

    const geometry = new THREE.PlaneGeometry(this.pixelWidth, this.pixelHeight);
    this.texture = new THREE.DataTexture(
      emuDisplayBuffer,
      this.pixelWidth,
      this.pixelHeight,
      THREE.RGBFormat,
    );
    this.texture.flipY = true;
    const material = new THREE.MeshBasicMaterial({ map: this.texture, side: THREE.DoubleSide });
    const plane = new THREE.Mesh(geometry, material);
    this.scene.add(plane);

    this.camera.position.set(0, 0, 25);
  }

  height() {
    return this.renderer.domElement.height;
  }

  width() {
    return this.renderer.domElement.width;
  }

  setDirtyFlag() {
    this.texture.needsUpdate = true;
  }

  render() {
    this.renderer.render(this.scene, this.camera);
  }
}
