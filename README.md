# power-k
## About
power-k is a tool of searching for vulnerable functions being used.   
It is used against a file system of a target system.  

## Usage 
After build using Cargo, it can be used like a below.  
`./power-k -m auto -d ./extracted_firmware_path -f system,strcpy,strncpy`

## Running mode
power-k give some funcitons and run in the following modes
* auto: Execute all executable functions in sequence.
* elf : Search for ELF files recursively in the passed directory.
* func: Search for the passed functions being used in ELF files.
* cert: Search for certification files.