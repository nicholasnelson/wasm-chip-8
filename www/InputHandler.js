const ALLOWED_KEYS = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A',
  'B', 'C', 'D', 'E', 'F'];

export default class InputHandler {
  constructor(cpu) {
    this.cpu = cpu;
    document.addEventListener('keydown', (event) => {
      const key = event.key.toUpperCase();
      if (ALLOWED_KEYS.includes(key)) {
        cpu.set_key_down(parseInt(key, 16));
      }
    });
    document.addEventListener('keyup', (event) => {
      const key = event.key.toUpperCase();
      if (ALLOWED_KEYS.includes(key)) {
        cpu.set_key_up(parseInt(key, 16));
      }
    });
  }
}
