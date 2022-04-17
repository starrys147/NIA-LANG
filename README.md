# NIA-LANG
nia lang is a language based on BrainF**k and 《伊甸nia园》 ("the nya Garden of Eden") series, which is a doujin video series of "Punishing: Gray Raven"

## syntax
same as Brainf**k
as long as you don't write other symbols and only contains 'nia' it will be fine.

## key words:
- `nia`: increase the value of current byte by one
  - brainf**k +
- `niA`: decrease the value of current byte by one
  - brains**k -
- `nIa`  bf ,
- `nIA`  bf .
- `Nia`  bf >
- `NiA`  bf <
- `NIa`  bf [
- `NIA`  bf ]

## example

> hello-world.nia:
```
niA 
NIa 
    niA niA niA niA niA niA niA 
    NiA 
    nia 
    Nia 
NIA 
NiA niA 
nIA 

niA 
NIa 
    niA 
    NiA nia nia nia nia nia 
    Nia 
NIA 
NiA nia nia 
nIA 

nia nia nia nia nia nia nia 
nIA 
nIA 

nia nia nia 
nIA 

NIa 
    niA niA niA NiA nia 
    Nia 
NIA 
NiA niA niA niA niA niA
nIA 

niA niA niA 
NIa 
    niA NiA nia nia nia Nia 
NIA 
NiA 
nIA 

niA 
NIa 
    niA niA niA NiA nia Nia 
NIA 
NiA niA niA niA 
nIA 

nia nia nia 
nIA 

niA niA niA niA niA niA 
nIA 

niA niA niA niA niA niA niA niA 
nIA 
```