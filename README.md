## Bitcoin Script Assembler

This is an experimental assembler for Bitcoin Script. It takes a well-formed Script file and outputs the assembled binary as hex-encoded text.

#### Sample

This command assembles text file __script.bcs__ into text file __script.bcb__.

    bitcoin-asm -i script.bcs -o script.bcb

__script.bcs__

    OP_DUP 
    OP_HASH160 
    788464014149d93b4a6135f3d665a0a2d743e6c3 
    OP_EQUALVERIFY 
    OP_CHECKSIG
 
__script.bcb__

    76A9788464014149D93B4A6135F3D665A0A2D743E6C388AC

----------


    USAGE:
      bitcoin-asm [global options]
    
    GLOBAL OPTIONS:
       -h, --help                         show the help message
       -i, --input <input>                set the input Bitcoin Script file.
       -o, --output <output>(optional)    sets the output file. Default: out.bcb
       -v, --version                      show the version message
