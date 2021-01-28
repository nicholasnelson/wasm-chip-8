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

function getWordDiv(word, active) {
  return `
    <div class="memory-byte-pair${active ? " memory-byte-pair-active" : ""}">
      ${word.toString(16).padStart(4, "0")}
    </div>
  `
}

function getMemoryRowDiv(memory, startAddress, pc) {
  let memoryU16 = new Uint16Array(memory.buffer);
  //console.log(memory);
  //console.log(memoryU16);
  return `
    <div class="row">
      ${getRowLabelDiv(startAddress)}
      ${memoryU16.reduce((acc, word, i) => acc + getWordDiv(word, startAddress + i*2 == pc), "")}
    </div>
      
  `
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
    const bounds = this.getDisplayBounds(pc);
    let output = "";
    // Loop over each row of BYTES_PER_ROW bytes
    for (let i = bounds.from; i < bounds.to; i += BYTES_PER_ROW) {
      output += getMemoryRowDiv(this.memory.slice(i, i + BYTES_PER_ROW), i, pc);
    }
    this.targetDiv.innerHTML = output;
  }

  getDisplayBounds(pc) {
    let rowStart = pc - pc % BYTES_PER_ROW;
    return {
      from: Math.max(rowStart - 56, 0),
      to: Math.min(rowStart + 64, 4096),
    }
  }


}