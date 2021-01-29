import { CPU } from "chip8";
import DisplayRenderer from "./DisplayRenderer";
import MemoryRenderer from "./MemoryRenderer";
import RegisterRenderer from "./RegisterRenderer";
import InputHandler from "./InputHandler";

export default class Chip8Controller {
  constructor(elements) {
    this.elements = elements;

    this.cpu = CPU.new();
    this.cpu.init_hex_sprites();
    this.cpu.load_image();

    this.displayRenderer = new DisplayRenderer(
      this.cpu.get_display_pointer(),
      this.elements.output.display);

    this.memoryRenderer = new MemoryRenderer(
      this.cpu.get_memory_pointer(),
      this.elements.output.memory,
    );

    this.registerRenderer = new RegisterRenderer(
      this.cpu.get_stack_pointer(),
      this.cpu.get_gpr_pointer(),
      () => ([
        { name: "i" , bytes: 4, value: this.cpu.get_i() },
        { name: "pc", bytes: 4, value: this.cpu.get_pc() },
        { name: "sp", bytes: 2, value: this.cpu.get_sp() },
        { name: "dt", bytes: 2, value: this.cpu.get_dt() },
        { name: "st", bytes: 2, value: this.cpu.get_st() },
      ]), this.elements.output.register);

    this.inputHandler = new InputHandler(this.cpu);

    this.lastTick = -1;
    this.targetTps = 10;
    this.running = false;

    this.setupButtons(elements.button);

    this.renderLoop = (timestamp) => {
      if (this.running && timestamp > this.lastTick + 1000 / this.targetTps) {
        this.stepCpu();
        this.render();
        this.lastTick = timestamp;
      }
    
      requestAnimationFrame(this.renderLoop);
    }

    this.render();
  }

  stepCpu() {
    this.cpu.tick();
    this.displayRenderer.setDirtyFlag();
  }

  render() {
    this.displayRenderer.render();
    this.registerRenderer.render();
    let pc = this.cpu.get_pc();
    this.memoryRenderer.render(pc);
  }

  toggleRun() {
    this.running = !this.running;
    if (this.running) {
      this.elements.button.start.classList.add("active");
    } else {
      this.elements.button.start.classList.remove("active");
    }
  }

  stepSim() {
    if (this.running) {
      this.toggleRun();
    }
    this.stepCpu();
    this.render();
  }

  reset() {
    alert("Not yet implemented!");
    // Push whatever image we last loaded to the CPU
    // Reset registers
  }

  load() {
    alert("Not yet implemented!");
    // Load a memory image from a file
    // Call this.reset() to handle resetting registers
  }

  setupButtons(buttons) {
    buttons.start.onclick = () => { this.toggleRun() };
    buttons.step.onclick = () => { this.stepSim() };
    buttons.reset.onclick = () => { this.reset() };
    buttons.load.onclick = () => { this.load() };
  }
}