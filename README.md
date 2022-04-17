# NIA-LANG
nia lang is a language based on BrainF**k and 《伊甸nia园》 ("the nya Garden of Eden") series, which is a doujin video series of "Punishing: Gray Raven"

## syntax
same as Brainf**k
as long as you don't write other symbols and only contains 'nia' it will be fine.

## key words:
- `nia`: increase the value of current byte by one
  - brainf**k `+`
- `niA`: decrease the value of current byte by one
  - brains**k `-`
- `nIa`  bf `,`
- `nIA`  bf `.`
- `Nia`  bf `>`
- `NiA`  bf `<`
- `NIA`  bf `[`
- `NIa`  bf `]`

## example

> hello-world.nia:
```
niA 
NIA 
    niA niA niA niA niA niA niA 
    NiA 
    nia 
    Nia 
NIa 
NiA niA 
nIA 

niA 
NIA 
    niA 
    NiA nia nia nia nia nia 
    Nia 
NIa 
NiA nia nia 
nIA 

nia nia nia nia nia nia nia 
nIA 
nIA 

nia nia nia 
nIA 

NIA 
    niA niA niA NiA nia 
    Nia 
NIa 
NiA niA niA niA niA niA
nIA 

niA niA niA 
NIA 
    niA NiA nia nia nia Nia 
NIa 
NiA 
nIA 

niA 
NIA 
    niA niA niA NiA nia Nia 
NIa 
NiA niA niA niA 
nIA 

nia nia nia 
nIA 

niA niA niA niA niA niA 
nIA 

niA niA niA niA niA niA niA niA 
nIA 
```