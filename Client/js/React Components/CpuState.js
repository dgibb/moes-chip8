import React from 'react';
import PropTypes from 'prop-types';
import { Register } from './Register';

function toHex(value) {
  let val = value.toString(16);
  while (val.length < 2) {
    val = '0' + val;
  }
  return (val.toUpperCase());
}

export class CpuState extends React.Component {
  render() {
    if (!this.props.romLoaded) {
      return (
        <div className="outer-container row grey">
          <div className="row">
            <div className="inner-container col-lg-8 col-md-10 col-sm-12 col-xs-12">
              <h2>About MOES:Chip8</h2>
              <p>MOES Chip8 Emulator is currently in its early stages. Until more progress is made you can attempt to execute ROM files here on the Chip8 to x86 dynamic recompiler being used as the emulators core. Please send me an email if you find any bugs or need to get in contact for any reason.</p>
            </div>
          </div>
        </div>
      );
    }

    const registers = this.props.registers.map(
      (register, i) => (
        <Register
          key={i}
          leftText={ 'r' + i.toString()}
          rightText={(register === 'N/A') ? 'N/A' : '0x' + toHex(register)}
        />
      ));

    return (
      <div className=" outer-container grey ">
        <div className="row" >
          <div id="register-box" className=" inner-container col-lg-6 col-md-8 col-sm-10 col-xs-10 bottom-border-1" >

            <div className="row bottom-border-2">
              <h1 className="register">Registers</h1>
            </div>

            <div className="row no-pad">
              {registers}
            </div>

          </div>
        </div>
      </div>
    );
  }
}

CpuState.propTypes = {
  registers: PropTypes.array.isRequired,
  romLoaded: PropTypes.bool.isRequired,
};
