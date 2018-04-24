import React from 'react';
import ReactDOM from 'react-dom';
import { Layout } from './js/React Components/Ch8 Layout';

const app = document.getElementById('app');
const socket = new WebSocket("ws://192.168.1.161:8080", ["protocolOne", "protocolTwo"]);


ReactDOM.render(
  <Layout socket={socket} />,
  app);
