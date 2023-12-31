# Skulkelys

I det siste har Nissen hatt et problem, nemlig at alvene hans har begynt å skulke. Dette er aldeles uhørt og ufint og uprofesjonelt og uproduktivt midt i høytiden hvor Nissen trenger det mest. Vet ikke ~~underklassen~~ alvene at han har forvaltningsansvar å ivareta?? Derfor har han kommet på en strategi som kan garantere han å finne ut av hvem det er som ikke er på jobb.

Strategien er slik: I gavefabrikken er det lange lange rekker med vinduer, alle med et lite julelys. Når hver alv kommer på jobb skal de tenne lyset i TO vinduer gitt av alvens ID (alvenes ID er langt høyere enn til at det er plass til ett vindu for hver alv). Reglene for hvilke vinduer de velger er definert av to enkle formler: (id * 2) % <antall vinduer> og (id + primtall nr. <id + 1>) % <antall vinduer>. På denne måten kan Nissen garantere at om han sjekker vinduene som hører til en gitt alv så kan han si med sikkerhet om den alven ikke er på jobb. Om alven faktisk er på jobb kan ikke denne metoden fortelle oss for en annen alv kan ha tent lys i ett (eller to) av vinduene, men det viktigste for Nissen er bare å straffe skulkerne.

Nissen sin strategi fungerer! Han drar mye glede ut av å finne og straffe skulkerne samtidig som han gjør sitt for bourgeoisiets fall. Mange alver slipper derimot unna med å skulke og er fornøyde med det. Grinchen, derimot, har rampestreker på gang: Etter at alle alvene er kommet på jobb sniker han seg rundt og blåser ut noen av lysene for å forvirre Nissen! Mange av alvene som før slapp unna med å skulke jobb vil ikke lenger være beskyttet av de litt spesielle vindureglene.

Gavefabrikken har 400 009 vinduer. I arkivet [input.tgz](input.tgz) finner du 3 filer: `alver_på_jobb.txt` inneholder alle alvene som kom på jobb, `alver_ikke_på_jobb.txt` inneholder alvene som skulker og `grinchen.txt` inneholder vinduene Grinchen slukket lysene i. Hvor mange av alvene som hadde sluppet unna med å skulke blir nå straffet som følge av Grinchen sine rampestreker?

Merk at alvenes ID-er starter på 0, men vi teller primtall fra 1.

## Eksempel

Si fabrikken har 7 vinduer, indeksert `0-6`. Alv `#0` og alv `#6` kom på jobb i dag, så de har tent lysene `(0*2) % 7 = 0, (0+2) % 7 = 2, (6*2) % 7 = 5 og (6+17) % 7 = 2`. Vinduene `0, 2, 5` har tente lys og `1, 3, 4, 6` har ikke.

Alv `#13` slipper unna med skulking fordi begge vinduene han skulle tent lys i er tent selv om han ikke er tilstede: `(13*2) % 7 = 5, (13+43) % 7 = 0`.

Alv #1 slipper ikke unna med skulking, for minst ett av vinduene han skulle tent står uten lys: `(1*2) % 7 = 2, (1+3) % 7 = 4`. Nissen kan se fra lysene i vinduene at han ikke er på jobb fordi vindu 4 står uten lys og det hadde vært tent dersom alv `#1` var på jobb.

Dersom Grinchen slukker lyset i vindu `#5` vil ikke lenger alv `#13` slippe unna med å skulke og svaret er da `1`.
