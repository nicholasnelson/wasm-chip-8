import Chip8Controller from './Chip8Controller';

const elements = {
  output: {
    display: document.getElementById('display-container'),
    memory: document.getElementById('memory-container'),
    register: {
      stack: document.getElementById('stack-table-body'),
      gpRegister: document.getElementById('gp-register-table-body'),
      sRegister: document.getElementById('special-register-table-body'),
    },
  },
  button: {
    turbo: document.getElementById('button-turbo'),
    start: document.getElementById('button-start'),
    step: document.getElementById('button-step'),
    reset: document.getElementById('button-reset'),
    load: document.getElementById('button-load'),
  },
  input: {
    romFile: document.getElementById('input-rom-file'),
  },
};

const controller = new Chip8Controller(elements);

requestAnimationFrame(controller.renderLoop);
