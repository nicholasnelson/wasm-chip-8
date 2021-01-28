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

    // Generate pixel vertices
    // const vertices = Float32Array.from(
    //   { length: this.pixelWidth * this.pixelHeight * 3 },
    //   (v, k) => {
    //     const pixel = Math.floor(k / 3);
    //     // Check if we're looking for the X, Y, or Z coord
    //     switch (k % 3) {
    //       case 0: // X
    //         return (pixel % this.pixelWidth) - this.pixelHeight / 2;
    //       case 1: // Y
    //         return this.pixelHeight / 2 - Math.floor(pixel / this.pixelWidth);
    //       default: // Z
    //         return 1;
    //     }
    //   },
    // );
    
    // const verticesBuffer = new THREE.BufferAttribute(vertices, 3);
    // const material = new THREE.PointsMaterial({ vertexColors: THREE.VertexColors });
    this.scene = new THREE.Scene();
    // this.pixelGeometry = new THREE.BufferGeometry();
    const emuDisplayBuffer = new Uint8Array(
      memory.buffer,
      displayPtr,
      this.pixelWidth * this.pixelHeight * 3);
    const colorBuffer = new THREE.BufferAttribute(emuDisplayBuffer, 3);
    // this.pixelGeometry.setAttribute('position', verticesBuffer);
    // this.pixelGeometry.setAttribute('color', colorBuffer);
    // const mesh = new THREE.Points(this.pixelGeometry, material);
    // this.scene.add(mesh);

    const geometry = new THREE.PlaneGeometry( this.pixelWidth, this.pixelHeight );
    this.texture = new THREE.DataTexture(emuDisplayBuffer, this.pixelWidth, this.pixelHeight, THREE.RGBFormat);
    this.texture.flipY = true;
    const material = new THREE.MeshBasicMaterial( {map: this.texture, side: THREE.DoubleSide} );
    const plane = new THREE.Mesh( geometry, material );
    this.scene.add( plane );

    this.camera.position.set(0, 0, 25);
  
    console.log(emuDisplayBuffer);
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

