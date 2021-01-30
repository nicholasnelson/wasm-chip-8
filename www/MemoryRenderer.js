// eslint-disable-next-line import/no-unresolved
import { memory } from 'chip8/chip_8_emu_bg';

const BYTES_PER_ROW = 8;

function getRowLabelDiv(address) {
  let start = address.toString(16).padStart(4, "0");
  let end = (address + BYTES_PER_ROW - 1).toString(16).padStart(4, "0");
  return `
      <div class="memory-address">
        ${start}..${end}:
      </div>
  `;
}

function getMemoryRowDiv(memory, startAddress, pc) {
  let html = [
    '<div class="row">',
    getRowLabelDiv(startAddress)];

  // Get the pc value relative to this memory block
  let offsetPC = pc - startAddress;
  // For each byte, add it to the html array
  for (let offset = 0; offset < memory.length; offset += 1) {
    html.push('<div class="memory-byte');
    // If this address is at the current PC or PC+1, mark it as active
    if (offset == offsetPC || offset == offsetPC + 1) {
      html.push(' memory-byte-active');
    }
    html.push('">', memory[offset].toString(16).padStart(2, '0'), '</div>');
  }
  html.push('</div>');
  return html.join('');
}

function getDisplayBounds(pc) {
  let rowStart = pc - pc % BYTES_PER_ROW;
  return {
    from: Math.max(rowStart - 56, 0),
    to: Math.min(rowStart + 64, 4096),
  };
}

export default class MemoryRenderer {
  constructor(memoryPtr, targetDiv, memoryLen = 4096) {
    this.targetDiv = targetDiv;
    this.memory = new Uint8Array(
      memory.buffer,
      memoryPtr,
      memoryLen);
  }

  render(pc) {
    const bounds = getDisplayBounds(pc);
    let output = [];
    // Loop over each row of BYTES_PER_ROW bytes
    for (let i = bounds.from; i < bounds.to; i += BYTES_PER_ROW) {
      output.push(getMemoryRowDiv(this.memory.slice(i, i + BYTES_PER_ROW), i, pc));
    }
    this.targetDiv.innerHTML = output.join('');
  }
}