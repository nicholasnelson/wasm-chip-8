import { CPU } from 'chip8/chip_8_emu';
import DisplayRenderer from './DisplayRenderer';
import MemoryRenderer from './MemoryRenderer';
import RegisterRenderer from './RegisterRenderer';
import InputHandler from './InputHandler';

export default class Chip8Controller {
  constructor(elements) {
    this.elements = elements;

    this.currentRom = [0x00, 0xe0, 0xa2, 0x48, 0x60, 0x00, 0x61, 0x1e, 0x62,
      0x00, 0xd2, 0x02, 0xd2, 0x12, 0x72, 0x08, 0x32, 0x40, 0x12, 0x0a, 0x60,
      0x00, 0x61, 0x3e, 0x62, 0x02, 0xa2, 0x4a, 0xd0, 0x2e, 0xd1, 0x2e, 0x72,
      0x0e, 0xd0, 0x2e, 0xd1, 0x2e, 0xa2, 0x58, 0x60, 0x0b, 0x61, 0x08, 0xd0,
      0x1f, 0x70, 0x0a, 0xa2, 0x67, 0xd0, 0x1f, 0x70, 0x0a, 0xa2, 0x76, 0xd0,
      0x1f, 0x70, 0x03, 0xa2, 0x85, 0xd0, 0x1f, 0x70, 0x0a, 0xa2, 0x94, 0xd0,
      0x1f, 0x12, 0x46, 0xff, 0xff, 0xc0, 0xc0, 0xc0, 0xc0, 0xc0, 0xc0, 0xc0,
      0xc0, 0xc0, 0xc0, 0xc0, 0xc0, 0xc0, 0xc0, 0xff, 0x80, 0x80, 0x80, 0x80,
      0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0xff, 0x81, 0x81,
      0x81, 0x81, 0x81, 0x81, 0x81, 0xff, 0x81, 0x81, 0x81, 0x81, 0x81, 0x81,
      0x81, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80,
      0x80, 0x80, 0x80, 0x80, 0xff, 0x81, 0x81, 0x81, 0x81, 0x81, 0x81, 0xff,
      0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0xff, 0x81, 0x81, 0x81, 0x81,
      0x81, 0x81, 0xff, 0x81, 0x81, 0x81, 0x81, 0x81, 0x81, 0xff, 0xff];

    this.cpu = CPU.new();

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

    this.ticksPerFrame = 10; // 10 ticks per frame * 60 fps = 600 tps
    this.running = true;
    this.turbo = true;

    this.setupButtons(elements.button, elements.input);

    this.renderLoop = () => {
      if (this.running) {
        this.stepCpu(this.ticksPerFrame * this.turbo ? 100 : 1);
      }
      requestAnimationFrame(this.renderLoop);
    };

    this.reset();
    this.render();
  }

  stepCpu(ticks = 1) {
    for (let i = 0; i < ticks; i++) {
      this.cpu.tick_timers(BigInt(Date.now()));
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

  toggleTurbo() {
    this.turbo = !this.turbo;
    if (this.turbo) {
      this.elements.button.turbo.classList.add('active');
    } else {
      this.elements.button.turbo.classList.remove('active');
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
    this.cpu.init_hex_sprites();
    this.cpu.load_program_memory(this.currentRom);
    this.render();
  }

  triggerLoad() {
    // Trigger the file input
    this.elements.input.romFile.click();
  }

  handleLoad() {
    if (this.turbo) {
      this.toggleTurbo();
    }

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
    buttons.turbo.addEventListener('click', () => { this.toggleTurbo(); });
    buttons.start.addEventListener('click', () => { this.toggleRun(); });
    buttons.step.addEventListener('click', () => { this.stepSim(); });
    buttons.reset.addEventListener('click', () => { this.reset(); });
    buttons.load.addEventListener('click', () => { this.triggerLoad(); });
    inputs.romFile.addEventListener('change', () => { this.handleLoad(); });
  }
}
