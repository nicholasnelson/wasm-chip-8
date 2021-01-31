// eslint-disable-next-line import/no-unresolved
import { memory } from 'chip8/chip_8_emu_bg';

function renderTable(tableData, bytes) {
  return tableData.reduce((acc, word, i) => `${acc
  }<tr>
      <td>${i.toString(16).toUpperCase()}</td>
      <td>0x${word.toString(16).padStart(bytes, '0').toUpperCase()}</td>
    </tr>`, '');
}

function renderLabelledTable(tableData) {
  return tableData.reduce((acc, register) => `${acc
  }
    <tr>
      <td>${register.name.toUpperCase()}</td>
      <td>0x${register.value.toString(16).padStart(register.bytes, '0').toUpperCase()}</td>
    </tr>
  `, '');
}

export default class RegisterRenderer {
  constructor(stackPointer, gpRegisterPointer, getRegisterValues, output) {
    this.output = output;
    this.gpRegisters = new Uint8Array(
      memory.buffer,
      gpRegisterPointer,
      16,
    );
    this.getRegisterValues = getRegisterValues;
    this.stack = new Uint16Array(
      memory.buffer,
      stackPointer,
      16,
    );
  }

  render() {
    this.output.stack.innerHTML = renderTable(this.stack, 4);
    this.output.gpRegister.innerHTML = renderTable(this.gpRegisters, 2);
    this.output.sRegister.innerHTML = renderLabelledTable(this.getRegisterValues());
  }
}
