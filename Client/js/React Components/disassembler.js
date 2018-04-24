// eslint-disable-file no-bitwise
// eslint-disable-file prefer-template

function toHex(value, length) {
  let val = value.toString(16);
  while (val.length < length) {
    val = '0' + val;
  }
  return (val.toUpperCase());
}

function disassemble(instruction, address) {
  switch (instruction & 0xF00) {
    case 0x1000:
      return 'jump ' + toHex(instruction & 0x0FFF, 4);

    case 0x3000:
      return 'je ' + toHex(address + 4, 4) + ', V' + toHex((instruction >> 8) & 0x0F, 1) + ' == ' + toHex(instruction & 0xFF, 2);

    case 0x4000:
      return 'jne ' + toHex(address + 4, 4) + ', V' + toHex((instruction >> 8) & 0x0F, 1) + ' == ' + toHex(instruction & 0xFF, 2);

    case 0x5000:
      return 'je ' + toHex(address + 4, 4) + ', V' + toHex((instruction >> 8) & 0x0F, 1) + ' == V' + toHex((instruction >> 4) & 0x0F, 1);

    case 0x6000:
      return 'mov V' + toHex((instruction >> 8) & 0x0F, 1) + ', ' + toHex(instruction & 0xFF, 2);

    case 0x7000:
      return 'addi V' + toHex((instruction >> 8) & 0x0F, 1) + ', ' +  toHex(instruction & 0xFF, 2);

    case 0x9000:
      return 'jne ' + toHex(address+4, 4) + ', V' + toHex((instruction >> 8) & 0x0F, 1) + ' == V' + toHex((instruction >> 4) & 0x0F, 1);

    case 0xA000:
      return 'set index ' + toHex(instruction & 0x0FFF, 4);

    case 0xB000:
      return 'jump ' + toHex(instruction & 0x0FFF, 4) + ' + V0';

    case 0xC000:
      return 'mov V' + toHex((instruction >> 8) & 0x0F, 1) + ' rand & ' + toHex(instruction & 0xFF, 2);

    case 0xD000:
      return 'Draw Sprite (V' + toHex((instruction >> 8) & 0x0F, 1) + ', V' + toHex((instruction >> 8) & 0x0F, 1) + '), width: ' + toHex((instruction) & 0x0F, 1);

    default: return 'unknown';
  }
}
