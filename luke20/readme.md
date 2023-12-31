# Nede, nede, nede i fangekjelleren

Alfarion, Nissens tidligere mest lojale alv, har fått nok av Nissen og drar inn i nissens fangekjeller (hvor han har hørt det også er drager!!) for å sette en stopper på juleeventyret.

Han kjenner til 3 terning-baserte angrep ([beskrevet med en spesiell notasjon](https://sophiehoulden.com/dice/documentation/notation.html)) som han tenker å bruke for å kverke Nissen:

1. Knivstikk i ryggen med ekstra dolkestikk: `2d6K1 + 2 + 1d4`, treffsikkerhet `1d20 + 8`
2. Sverdslag: `1d8 + 5`, treffsikkerhet `1d20 + 6`
3. Økseslag: `2d10KL1 + 6`, treffsikkerhet `1d20 + 3`

Alfarion bruker hvert angrep i tur helt til Nissen er tom for helsepoeng. Nissen er ganske sleip av seg, så før hvert angrep må han sjekke om angrepet faktisk treffer. Dette gjør han ved å kaste terning for treffsikkerhet mot Nissens sleiphet som er `18`, altså må kastet for treffsikkerhet være minst `18` for å gjøre skade. Når han vet at angrepet treffer kan han kaste for hvor mye skade Nissen påføres.

Hvilke terningkaste Alfarion gjør finner du i [dette arkivet](terningkast.tgz). Hver linje beskriver et terningkast, når du når enden av fila starter du fra topp igjen.

Nissen har `10 000 000` helsepoeng. Hvor mange angrep må til for å drepe han?
Eksempel

De første angrepene ser slik ut:

```
Angrep 1 (våpen 1):
  Kaster d20 for 12; 12 + 8 >= 18 = T
  Kaster 2d6 for [1, 6], d4 for 4; 6 + 2 + 4 = 12 skade
Angrep 2 (våpen 2):
  Kaster d20 for 1; 1 + 6 >= 18 = F
Angrep 3 (våpen 3):
  Kaster d20 for 3; 3 + 3 >= 18 = F
Angrep 4 (våpen 1):
  Kaster d20 for 5; 5 + 8 >= 18 = F
Angrep 5 (våpen 2):
  Kaster d20 for 14; 14 + 6 >= 18 = T
  Kaster d8 for 3; 3 + 5 = 8 skade
Angrep 6 (våpen 3):
  Kaster d20 for 1; 1 + 3 >= 18 = F
...
```
