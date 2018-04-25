# MOES-chip8
Server based dynamic recompiler and chip8 emulator with websockets written in Rust

Part of the Multiplayer Online Emulator Suite Project

API Documentation
=================

The API is currently incomplete as the project is still a work in progress, routes currently inplemented are used mostly for development and debugging

**Routes**
  
  These are standard http routes that the server uses to execute commands.

* /sendRom - POST

  This is the route that the player uses to send a ROM file to the server for execution 
  
  Arguments: ROM File
 
  Return: { valid: bool }
    * true if rom was succesfully loaded into emulator 
  
* /runblock - GET

  This sends a signal to the server to run a block of recompiled code, and return the state of the emulator after execution has finished 
  
  Arguments: none
  
  return: {  pc: int, 
             index: int, 
             gpr: [16]int, 
             stack:[16]int, 
             blocks:[ ]int, 
           }
  
