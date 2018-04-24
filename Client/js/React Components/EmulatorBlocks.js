import React from 'react';
import PropTypes from 'prop-types';
import { Register } from './Register';
import { disassemble } from './disassembler'

function toHex(value, length) {
  let val = value.toString(16);
  while (val.length < length) {
    val = '0' + val;
  }
  return (val.toUpperCase());
}

export class EmulatorBlocks extends React.Component {
  constructor() {
    super();
    this.state = {
      currentBlock: 0,
    };
  }

  setBlock(address) {
    this.setstate({
      currentBlock: address,
    });
  }

  render() {
    const blocks = this.props.blocks.map(
      (block, i) => (
        <Register
          key={i}
          address={'0x' + toHex(block, 4)}
          onClick={this.seBlock(block)}
        />
      ));

    const instructions = (
      (instruction, i) => (
        <instruction
          key={i}
          address={'0x' + (this.state.currentBlock + i)}
          hex={toHex(this.props.rom[this.state.currentBlock - 0x200 + (2 * i)], 4)}
          disassembly={disassemble(this.props.rom[this.state.currentBlock - 0x200 + (2 * i)])}
        />
      ));

    return (
      <div className=" outer-container white ">
        <div className="row" >
          <div id="register-box" className=" inner-container col-lg-8 col-md-10 col-sm-12 col-xs-12 bottom-border-1" >

            <div className="row bottom-border-2">
              <h1 className="register">Blocks</h1>
            </div>

            <div>
              {blocks}
            </div>

            <div className="row no-pad">
              {instructions}
            </div>

          </div>
        </div>
      </div>
    );
  }
}

EmulatorBlocks.propTypes = {
  blocks: PropTypes.array.isRequired,
  rom: PropTypes.object.isRequired,
};
