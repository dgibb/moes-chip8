import React from 'react';
import PropTypes from 'prop-types';

export class Register extends React.Component {

  render() {
    const regClass = 'register';

    return (
      <div className="col-lg-12 col-md-12 col-sm-12 col-xs-12 no-gutters">
        <div className="col-lg-2 col-md-2 col-sm-2 col-xs-2 left-column bottom-border-1">
          <p className="register">{this.props.address}</p>
        </div>
        <div className="col-lg-5 col-md-5 col-sm-5 col-xs-5 bottom-border-1">
          <p className={regClass}>{this.props.hex}</p>
        </div>
        <div className="col-lg-10 col-md-10 col-sm-10 col-xs-10 bottom-border-1">
          <p className={regClass}>{this.props.disassembly}</p>
        </div>
      </div>
    );
  }
}

Register.propTypes = {
  address: PropTypes.string.isRequired,
  hex: PropTypes.string.isRequired,
  disassembly: PropTypes.string.isRequired,
};
