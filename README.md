# Atta
Atta is a WIP of a node package manager written in rust.

## Motivations
Because using Javascript to create a package manager makes no sense whatsoever, Disk IO with node are way too slow.  
Also because neither yarn or npm make use of a store, instead they copy the same package on the disk a dozillion time. Only [pnpm](https://github.com/pnpm/pnpm) does it.