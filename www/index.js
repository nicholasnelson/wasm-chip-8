import Chip8Controller from "./Chip8Controller";

let elements = {
  output: {
    display: document.getElementById("display-container"),
    memory: document.getElementById("memory-container"),
    register: {
      stack: document.getElementById("stack-table-body"),
      gpRegister: document.getElementById("gp-register-table-body"),
      sRegister: document.getElementById("special-register-table-body"),
    },
  },
  button: {
    start: document.getElementById("button-start"),
    step: document.getElementById("button-step"),
    reset: document.getElementById("button-reset"),
    load: document.getElementById("button-load"),
  }
};

const controller = new Chip8Controller(elements);

requestAnimationFrame(controller.renderLoop);
