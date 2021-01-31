import { CPU } from 'chip8/chip_8_emu';
import DisplayRenderer from './DisplayRenderer';
import MemoryRenderer from './MemoryRenderer';
import RegisterRenderer from './RegisterRenderer';
import InputHandler from './InputHandler';

export default class Chip8Controller {
  constructor(elements) {
    this.elements = elements;

    this.ticksPerFrame = 10; // 10 ticks per frame @ 60fps = 600Hz

    this.cpu = CPU.new();
    this.cpu.init_hex_sprites();

    this.displayRenderer = new DisplayRenderer(
      this.cpu.get_display_pointer(),
      this.elements.output.display,
    );

    this.memoryRenderer = new MemoryRenderer(
      this.cpu.get_memory_pointer(),
      this.elements.output.memory,
    );

    this.registerRenderer = new RegisterRenderer(
      this.cpu.get_stack_pointer(),
      this.cpu.get_gpr_pointer(),
      () => ([
        { name: 'i', bytes: 4, value: this.cpu.get_i() },
        { name: 'pc', bytes: 4, value: this.cpu.get_pc() },
        { name: 'sp', bytes: 2, value: this.cpu.get_sp() },
        { name: 'dt', bytes: 2, value: this.cpu.get_dt() },
        { name: 'st', bytes: 2, value: this.cpu.get_st() },
      ]), this.elements.output.register,
    );

    this.inputHandler = new InputHandler(this.cpu);

    this.lastTick = -1;
    this.targetTps = 10;
    this.running = false;

    this.setupButtons(elements.button, elements.input);

    this.renderLoop = (timestamp) => {
      if (this.running && timestamp > this.lastTick + 1000 / this.targetTps) {
        this.stepCpu(this.ticksPerFrame);
        this.lastTick = timestamp;
      }

      requestAnimationFrame(this.renderLoop);
    };

    this.render();
  }

  stepCpu(ticks = 1) {
    for (let i = 0; i < ticks; i++) {
      this.cpu.tick();
    }
    this.displayRenderer.setDirtyFlag();
    this.render();
  }

  render() {
    this.displayRenderer.render();
    this.registerRenderer.render();
    const pc = this.cpu.get_pc();
    this.memoryRenderer.render(pc);
  }

  toggleRun() {
    this.running = !this.running;
    if (this.running) {
      this.elements.button.start.classList.add('active');
    } else {
      this.elements.button.start.classList.remove('active');
    }
  }

  stepSim() {
    if (this.running) {
      this.toggleRun();
    }
    this.stepCpu();
  }

  reset() {
    this.cpu.reset();
    this.cpu.load_program_memory(this.currentRom);
    this.render();
  }

  triggerLoad() {
    // Trigger the file input
    this.elements.input.romFile.click();
  }

  handleLoad() {
    const romFile = this.elements.input.romFile.files[0];
    // Read the file
    romFile.arrayBuffer().then((result) => {
      this.currentRom = new Uint8Array(result);
      this.reset();
    });

    this.cpu.load_program_memory([0xF1, 0x0A, 0xE1, 0x9E, 0x70, 0x01, 0x12, 0x02]);
    // Call this.reset() to handle resetting registers
  }

  setupButtons(buttons, inputs) {
    buttons.start.addEventListener('click', () => { this.toggleRun(); });
    buttons.step.addEventListener('click', () => { this.stepSim(); });
    buttons.reset.addEventListener('click', () => { this.reset(); });
    buttons.load.addEventListener('click', () => { this.triggerLoad(); });
    inputs.romFile.addEventListener('change', () => { this.handleLoad(); });
  }
}
