# Internprising

Grunnet mange påfunn fra Grinchen og problemer med fabrikken så har ikke alvene klart å lage nok gaver til juleaften. Heldigvis har alverådet et nødlager med penger i tilfelle noe slik skulle skje! Nissen går derfor til innkjøp av gaver fra Sydpolen. Men nok en gang, så mistenker Nissen at Grinchen har vært på ferde. Han lurer på om Grinchen kanskje har vært inne og tuklet med prisene under innkjøp av gavene. Nissen henter ut et utdrag av transaksjonene og begynner å lete.

Hver linje i filen har dette formatet:

```
tittel;pris;hash
```
Hashingen fungerer slik:

1. Hent ut summen av tittelens sine bokstaver plass i alfabetet. Ignorer altså alt som ikke er bokstaver. Det er det norske alfabetet som brukes. a=1, b=2 osv..
2. Gang med pris
3. Modulo resultatet fra steg 2 med `0xbeef`
For å kontrollere om en transaksjon er gyldig må du sammenligne oppgitt hash med resultatet fra stegene nevnt over.

I [denne](src/transaksjoner.txt) filen finner du utdraget av transaksjoner.

Kan du finne ut hvilke transaksjoner som er ugyldige? Svar med første bokstaven i hver av gavene til de ugyldige transksjonene, i rekkefølgen de dukker opp i filen.

## Eksempel:
```
Factorio;1535;35787
Harry Potter 1;321;89723                  - korrupt
Skateboard;5207;11082
En krukke med ingenting;134528;34980      - korrupt
iPod;9423;3298709                         - korrupt
```
Blir til `hei`.
